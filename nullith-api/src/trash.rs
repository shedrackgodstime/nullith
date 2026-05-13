use http_body_util::BodyExt;
use serde::Deserialize;
use tokio_postgres::NoTls;
use vercel_runtime::{run, service_fn, Error, Request, Response};

#[derive(Deserialize)]
struct TrashInput {
    data_key: String,
    data_type: String,
    content: String,
    created_at: f64,
    expires_at: f64,
}

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

    if req.uri().path().ends_with("trash") && req.method() == "GET" {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let rows = client
            .query(
                "SELECT id, data_key, data_type, content, created_at, expires_at FROM trash WHERE expires_at > $1 ORDER BY created_at DESC",
                &[&now],
            )
            .await
            .map_err(|e| e.to_string())?;

        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|r| {
                serde_json::json!({
                    "id": r.get::<_, i64>(0),
                    "data_key": r.get::<_, String>(1),
                    "data_type": r.get::<_, String>(2),
                    "content": r.get::<_, String>(3),
                    "created_at": r.get::<_, f64>(4),
                    "expires_at": r.get::<_, f64>(5)
                })
            })
            .collect();

        let body = serde_json::to_string(&items).map_err(|e| e.to_string())?;
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(body)?);
    }

    if req.uri().path().ends_with("trash") && req.method() == "POST" {
        let body = req
            .into_body()
            .collect()
            .await
            .map_err(|e| e.to_string())?
            .to_bytes();
        let input: TrashInput = serde_json::from_slice(&body).map_err(|e| e.to_string())?;

        client.execute(
            "INSERT INTO trash (data_key, data_type, content, created_at, expires_at) VALUES ($1, $2, $3, $4, $5)",
            &[
                &input.data_key,
                &input.data_type,
                &input.content,
                &input.created_at,
                &input.expires_at,
            ],
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
