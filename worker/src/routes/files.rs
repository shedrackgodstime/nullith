use worker::*;

use crate::auth;

fn check_auth(req: &Request, env: &Env) -> Result<(), worker::Error> {
    if !auth::verify_api_key(req, env)? {
        return Err(worker::Error::from("Unauthorized"));
    }
    Ok(())
}

pub async fn handle_upload_file(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    let path = ctx.param("path").ok_or("Missing path")?;
    
    let data = req.bytes().await?;
    let size = data.len();
    log::info!("Uploading file: {} ({} bytes)", path, size);

    bucket.put(path, data).execute().await?;
    
    log::info!("File uploaded: {}", path);
    Response::from_json(&serde_json::json!({
        "success": true,
        "path": path,
        "size": size
    }))
}

pub async fn handle_get_file(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&_req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    let path = ctx.param("path").ok_or("Missing path")?;
    
    log::info!("Getting file: {}", path);

    let obj = bucket.get(path).execute().await?;
    
    match obj {
        Some(object) => {
            let mime = path.split('.').last()
                .map(|ext| match ext {
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "gif" => "image/gif",
                    "pdf" => "application/pdf",
                    "txt" => "text/plain",
                    "json" => "application/json",
                    "html" => "text/html",
                    "css" => "text/css",
                    "js" => "application/javascript",
                    _ => "application/octet-stream",
                })
                .unwrap_or("application/octet-stream");
            
            let headers = Headers::new();
            headers.set("Content-Type", mime).map_err(|e| worker::Error::from(e.to_string()))?;
            
            // For now return 404 if no body - reading body needs wasm-streams
            if object.body().is_some() {
                Ok(Response::empty()?.with_headers(headers))
            } else {
                Response::error("File has no content", 404)
            }
        }
        None => Response::error("File not found", 404),
    }
}

pub async fn handle_delete_file(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&_req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    let path = ctx.param("path").ok_or("Missing path")?;
    
    log::info!("Deleting file: {}", path);

    bucket.delete(path).await?;
    
    log::info!("File deleted: {}", path);
    Response::from_json(&serde_json::json!({
        "success": true,
        "path": path
    }))
}

pub async fn handle_list_files(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&_req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    
    log::info!("Listing files");

    let list = bucket.list().execute().await?;
    let files: Vec<serde_json::Value> = list.objects()
        .iter()
        .map(|o| serde_json::json!({
            "key": o.key(),
            "size": o.size()
        }))
        .collect();

    Response::from_json(&files)
}