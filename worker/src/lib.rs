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
        .get_async("/notes", routes::notes::handle_get_notes)
        .get_async("/notes/:key", routes::notes::handle_get_note)
        .post_async("/notes/:key", routes::notes::handle_set_note)
        .put_async("/notes/:key", routes::notes::handle_set_note)
        .delete_async("/notes/:key", routes::notes::handle_delete_note);
        // .get_async("/files", routes::files::handle_list_files)
        // .get_async("/files/*path", routes::files::handle_get_file)
        // .put_async("/files/*path", routes::files::handle_upload_file)
        // .delete_async("/files/*path", routes::files::handle_delete_file);

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