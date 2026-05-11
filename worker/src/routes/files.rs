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
    log::info!("Uploading file: {} ({} bytes)", path, data.len());

    let put = bucket.put(path, data);
    put.await?;

    log::info!("File uploaded: {}", path);
    Response::from_json(&serde_json::json!({
        "success": true,
        "path": path,
        "size": data.len()
    }))
}

pub async fn handle_get_file(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&_req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    let path = ctx.param("path").ok_or("Missing path")?;
    
    log::info!("Getting file: {}", path);

    let get = bucket.get(path);
    let obj: Option<worker::R2Object> = get.await?;
    
    match obj {
        Some(object) => {
            let read = object.read();
            let data: Vec<u8> = read.await?;
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
            
            let mut headers = Headers::new();
            headers.set("Content-Type", mime).map_err(|e| worker::Error::from(e.to_string()))?;
            headers.set("Content-Length", &data.len().to_string()).map_err(|e| worker::Error::from(e.to_string()))?;
            
            Ok(Response::from_bytes(data)?.with_headers(headers))
        }
        None => Response::error("File not found", 404),
    }
}

pub async fn handle_delete_file(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_auth(&_req, &ctx.env)?;
    
    let bucket = ctx.env.bucket("FILES")?;
    let path = ctx.param("path").ok_or("Missing path")?;
    
    log::info!("Deleting file: {}", path);

    let delete = bucket.delete(path);
    delete.await?;
    
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

    let list = bucket.list();
    let list_result: worker::R2List = list.await?;
    let files: Vec<serde_json::Value> = list_result.objects
        .iter()
        .map(|o| serde_json::json!({
            "key": o.key,
            "size": o.size,
            "uploaded": o.uploaded
        }))
        .collect();

    Response::from_json(&files)
}