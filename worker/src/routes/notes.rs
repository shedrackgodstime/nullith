use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use worker::*;

fn now_millis() -> String {
    js_sys::Date::now().to_string()
}

fn js_value_to_string(v: &JsValue) -> String {
    v.as_string().unwrap_or_default()
}

fn js_value_to_f64(v: &JsValue) -> f64 {
    v.as_f64().unwrap_or(0.0)
}

pub async fn handle_get_notes(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT key, value, create_at, update_at FROM notes ORDER BY update_at DESC LIMIT 50")?;
    let result = stmt.raw_js_value().await?;
    
    let notes: Vec<serde_json::Value> = result
        .into_iter()
        .map(|row| {
            let arr = row.unchecked_ref::<js_sys::Array>();
            serde_json::json!({
                "key": js_value_to_string(&arr.get(0)),
                "value": js_value_to_string(&arr.get(1)),
                "create_at": js_value_to_f64(&arr.get(2)),
                "update_at": js_value_to_f64(&arr.get(3))
            })
        })
        .collect();

    Response::from_json(&notes)
}

pub async fn handle_get_note(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;
    
    let d1 = ctx.env.d1("DB")?;
    let stmt = d1.prepare("SELECT key, value, create_at, update_at FROM notes WHERE key = ?")?;
    let bound = stmt.bind(&[JsValue::from(key)])?;
    let result = bound.raw_js_value().await?;

    if result.is_empty() {
        return Response::error(format!("Note '{}' not found", key), 404);
    }
    
    let arr = result[0].unchecked_ref::<js_sys::Array>();
    let note = serde_json::json!({
        "key": js_value_to_string(&arr.get(0)),
        "value": js_value_to_string(&arr.get(1)),
        "create_at": js_value_to_f64(&arr.get(2)),
        "update_at": js_value_to_f64(&arr.get(3))
    });

    Response::from_json(&note)
}

pub async fn handle_set_note(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;
    let body: serde_json::Value = req.json().await?;
    let value = body.get("value").and_then(|v| v.as_str()).unwrap_or("");
    let now = now_millis();

    let d1 = ctx.env.d1("DB")?;
    
    let check = d1.prepare("SELECT key FROM notes WHERE key = ?")?;
    let bound_check = check.bind(&[JsValue::from(key)])?;
    let exists = bound_check.raw_js_value().await?;
    
    if !exists.is_empty() {
        let stmt = d1.prepare("UPDATE notes SET value = ?, update_at = ? WHERE key = ?")?;
        let bound = stmt.bind(&[JsValue::from(value), JsValue::from(now.clone()), JsValue::from(key)])?;
        bound.run().await?;
    } else {
        let stmt = d1.prepare("INSERT INTO notes (key, value, create_at, update_at) VALUES (?, ?, ?, ?)")?;
        let bound = stmt.bind(&[
            JsValue::from(key),
            JsValue::from(value),
            JsValue::from(now.clone()),
            JsValue::from(now.clone())
        ])?;
        bound.run().await?;
    }

    let response = serde_json::json!({
        "key": key,
        "value": value,
        "create_at": now,
        "update_at": now
    });
    
    Response::from_json(&response)
}

pub async fn handle_delete_note(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let key = ctx.param("key").ok_or("Missing key")?;

    let d1 = ctx.env.d1("DB")?;
    
    let check = d1.prepare("SELECT key FROM notes WHERE key = ?")?;
    let bound_check = check.bind(&[JsValue::from(key)])?;
    let exists = bound_check.raw_js_value().await?;

    if exists.is_empty() {
        return Response::error(format!("Note '{}' not found", key), 404);
    }

    let stmt = d1.prepare("DELETE FROM notes WHERE key = ?")?;
    let bound = stmt.bind(&[JsValue::from(key)])?;
    bound.run().await?;

    Response::from_json(&serde_json::json!({"success": true}))
}