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

    if req.method() == "DELETE" {
        let path = req.uri().path();
        let id_str = path.strip_prefix("/trash/").unwrap_or("");
        let id: i64 = id_str.parse().map_err(|_| "Invalid ID")?;

        client
            .execute("DELETE FROM trash WHERE id = $1", &[&id])
            .await
            .map_err(|e| e.to_string())?;

        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"status": "deleted"}"#.to_string())?);
    }

    Ok(Response::builder()
        .status(404)
        .body("Not found".to_string())?)
}
