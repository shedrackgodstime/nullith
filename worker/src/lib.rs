use worker::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

fn now_millis() -> String {
    js_sys::Date::now().to_string()
}

// ============ NOTES ============

async fn handle_get_notes(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT key, value, create_at, update_at FROM notes ORDER BY update_at DESC LIMIT 50");
    let result = stmt.raw_js_value().await?;
    
    let notes: Vec<serde_json::Value> = result.into_iter().map(|row| {
        let arr = row.unchecked_ref::<js_sys::Array>();
        serde_json::json!({
            "key": arr.get(0).as_string().unwrap_or_default(),
            "value": arr.get(1).as_string().unwrap_or_default(),
            "create_at": arr.get(2).as_f64().unwrap_or(0.0),
            "update_at": arr.get(3).as_f64().unwrap_or(0.0)
        })
    }).collect();

    Response::from_json(&notes)
}

async fn handle_get_note(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;
    
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT key, value, create_at, update_at FROM notes WHERE key = ?");
    let bound = stmt.bind(&[JsValue::from(key)])?;
    let result = bound.raw_js_value().await?;

    if result.is_empty() {
        return Response::error("Not found", 404);
    }
    
    let arr = result[0].unchecked_ref::<js_sys::Array>();
    Response::from_json(&serde_json::json!({
        "key": arr.get(0).as_string().unwrap_or_default(),
        "value": arr.get(1).as_string().unwrap_or_default(),
        "create_at": arr.get(2).as_f64().unwrap_or(0.0),
        "update_at": arr.get(3).as_f64().unwrap_or(0.0)
    }))
}

async fn handle_set_note(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;
    let body: serde_json::Value = req.json().await?;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or("");
    let now = now_millis();

    let d1 = ctx.env.d1("DB")?;
    
    let check = d1.prepare("SELECT key FROM notes WHERE key = ?");
    let bound_check = check.bind(&[JsValue::from(key)])?;
    let exists = bound_check.raw_js_value().await?;
    
    if !exists.is_empty() {
        let stmt = d1.prepare("UPDATE notes SET value = ?, update_at = ? WHERE key = ?");
        let bound = stmt.bind(&[JsValue::from(value), JsValue::from(now.clone()), JsValue::from(key)])?;
        bound.run().await?;
    } else {
        let stmt = d1.prepare("INSERT INTO notes (key, value, create_at, update_at) VALUES (?, ?, ?, ?)");
        let bound = stmt.bind(&[JsValue::from(key), JsValue::from(value), JsValue::from(now.clone()), JsValue::from(now.clone())])?;
        bound.run().await?;
    }

    Response::from_json(&serde_json::json!({
        "key": key,
        "value": value,
        "create_at": now,
        "update_at": now
    }))
}

async fn handle_delete_note(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;

    let d1 = ctx.env.d1("DB")?;
    let check = d1.prepare("SELECT key FROM notes WHERE key = ?");
    let bound_check = check.bind(&[JsValue::from(key)])?;
    let exists = bound_check.raw_js_value().await?;

    if exists.is_empty() {
        return Response::error("Not found", 404);
    }

    let stmt = d1.prepare("DELETE FROM notes WHERE key = ?");
    let bound = stmt.bind(&[JsValue::from(key)])?;
    bound.run().await?;

    Response::from_json(&serde_json::json!({"success": true}))
}

// ============ ROOT ============

async fn handle_root(_: Request, _: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&serde_json::json!({
        "name": "Nullith API",
        "version": "1.0.0"
    }))
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/", handle_root)
        .get_async("/notes", handle_get_notes)
        .get_async("/notes/:key", handle_get_note)
        .post_async("/notes/:key", handle_set_note)
        .put_async("/notes/:key", handle_set_note)
        .delete_async("/notes/:key", handle_delete_note)
        .run(req, env)
        .await
}