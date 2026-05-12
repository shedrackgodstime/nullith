use http_body_util::BodyExt;
use serde_json::Value;
use tokio_postgres::NoTls;
use vercel_runtime::{run, service_fn, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(req: Request) -> Result<Response<String>, Error> {
    let db_url = std::env::var("NEON_DB_URL").map_err(|_| "NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
        .await
        .map_err(|e| e.to_string())?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    if req.uri().path().ends_with("logs") && req.method() == "GET" {
        let rows = client
            .query(
                "SELECT id, level, message, created_at FROM logs ORDER BY created_at DESC LIMIT 100",
                &[],
            )
            .await
            .map_err(|e| e.to_string())?;

        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|r| {
                serde_json::json!({
                    "id": r.get::<_, i64>(0),
                    "level": r.get::<_, String>(1),
                    "message": r.get::<_, String>(2),
                    "created_at": r.get::<_, f64>(3)
                })
            })
            .collect();

        let body = serde_json::to_string(&items).map_err(|e| e.to_string())?;
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(body)?);
    }

    if req.uri().path().ends_with("logs") && req.method() == "POST" {
        let body_bytes = req
            .into_body()
            .collect()
            .await
            .map_err(|e| e.to_string())?
            .to_bytes();
        let payload: Value = serde_json::from_slice(&body_bytes).map_err(|e| e.to_string())?;

        let level = payload
            .get("level")
            .and_then(|v| v.as_str())
            .unwrap_or("info");
        let message = payload
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let created_at = payload
            .get("created_at")
            .and_then(|v| v.as_f64())
            .unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            });

        client
            .execute(
                "INSERT INTO logs (level, message, created_at) VALUES ($1, $2, $3)",
                &[&level, &message, &created_at],
            )
            .await
            .map_err(|e| e.to_string())?;

        return Ok(Response::builder()
            .status(201)
            .header("Content-Type", "application/json")
            .body(r#"{"status": "ok"}"#.to_string())?);
    }

    Ok(Response::builder()
        .status(404)
        .body("Not found".to_string())?)
}
