use reqwest::Client;
use vercel_runtime::{run, service_fn, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_req: Request) -> Result<Response<String>, Error> {
    let worker_url = std::env::var("NULLITH_URL")
        .unwrap_or_else(|_| "https://nullith-worker.shedrackgodstime.workers.dev".to_string());
    let api_key = std::env::var("NULLITH_API_KEY").map_err(|_| "NULLITH_API_KEY not set")?;

    let client = Client::new();
    let url = format!("{}/files", worker_url);

    let response = client
        .get(&url)
        .header("X-API-Key", &api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(body)?)
}
