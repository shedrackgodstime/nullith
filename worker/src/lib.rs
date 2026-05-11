mod auth;
mod error;
mod logging;
mod models;
mod routes;
mod utils;

use models::HealthCheck;
use worker::*;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new()
        .get_async("/", handle_root)
        .get_async("/debug/secret", handle_debug_secret)
        .post_async("/admin/rotate-key", routes::admin::handle_rotate_key)
        .get_async("/notes", routes::notes::handle_get_notes)
        .get_async("/notes/:key", routes::notes::handle_get_note)
        .post_async("/notes/:key", routes::notes::handle_set_note)
        .put_async("/notes/:key", routes::notes::handle_set_note)
        .delete_async("/notes/:key", routes::notes::handle_delete_note)
        .get_async("/files", routes::files::handle_list_files)
        .get_async("/files/*path", routes::files::handle_get_file)
        .put_async("/files/*path", routes::files::handle_upload_file)
        .delete_async("/files/*path", routes::files::handle_delete_file);

    router.run(req, env).await
}

async fn handle_root(_: Request, _: RouteContext<()>) -> worker::Result<Response> {
    log::info!("GET / - health check");
    Response::from_json(&HealthCheck {
        name: "Nullith API".to_string(),
        version: "1.0.0".to_string(),
        status: "running".to_string(),
    })
}

async fn handle_debug_secret(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    match ctx.env.secret("API_KEY") {
        Ok(key) => {
            let val = key.to_string();
            log::info!("Secret found, length: {}", val.len());
            Response::from_json(&serde_json::json!({
                "status": "found",
                "length": val.len()
            }))
        }
        Err(e) => {
            log::warn!("Secret error: {:?}", e);
            Response::from_json(&serde_json::json!({
                "status": "error",
                "error": format!("{:?}", e)
            }))
        }
    }
}
