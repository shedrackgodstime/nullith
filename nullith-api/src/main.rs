use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

#[derive(Clone)]
struct AppState {
    worker_url: String,
    api_key: String,
    db_url: Option<String>,
    client: Client,
}

#[derive(Serialize)]
struct TrashItem {
    id: i64,
    data_key: String,
    data_type: String,
    content: String,
    created_at: f64,
    expires_at: f64,
}

#[derive(Deserialize)]
struct TrashInput {
    data_key: String,
    data_type: String,
    content: String,
    created_at: f64,
    expires_at: f64,
}

#[derive(Serialize)]
struct LogItem {
    id: i64,
    level: String,
    message: String,
    created_at: f64,
}

async fn get_notes(State(state): State<Arc<Mutex<AppState>>>) -> Result<String, String> {
    let state = state.lock().await;
    let url = format!("{}/notes", state.worker_url);
    let body = state
        .client
        .get(&url)
        .header("X-API-Key", &state.api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(body)
}

async fn get_note(
    Path(key): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let state = state.lock().await;
    let url = format!("{}/notes/{}", state.worker_url, key);
    let body = state
        .client
        .get(&url)
        .header("X-API-Key", &state.api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(body)
}

async fn list_files(State(state): State<Arc<Mutex<AppState>>>) -> Result<String, String> {
    let state = state.lock().await;
    let url = format!("{}/files", state.worker_url);
    let body = state
        .client
        .get(&url)
        .header("X-API-Key", &state.api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(body)
}

async fn get_file(
    Path(path): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let state = state.lock().await;
    let url = format!("{}/files/{}", state.worker_url, path);
    let body = state
        .client
        .get(&url)
        .header("X-API-Key", &state.api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(body)
}

async fn list_trash(State(state): State<Arc<Mutex<AppState>>>) -> Result<Json<Vec<TrashItem>>, String> {
    let state = state.lock().await;
    let db_url = state.db_url.as_ref().ok_or("NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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

    let items: Vec<TrashItem> = rows
        .iter()
        .map(|r| TrashItem {
            id: r.get(0),
            data_key: r.get(1),
            data_type: r.get(2),
            content: r.get(3),
            created_at: r.get(4),
            expires_at: r.get(5),
        })
        .collect();

    Ok(Json(items))
}

async fn add_trash(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TrashInput>,
) -> Result<String, String> {
    let state = state.lock().await;
    let db_url = state.db_url.as_ref().ok_or("NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .execute(
            "INSERT INTO trash (data_key, data_type, content, created_at, expires_at) VALUES ($1, $2, $3, $4, $5)",
            &[&payload.data_key, &payload.data_type, &payload.content, &payload.created_at, &payload.expires_at],
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(r#"{"status": "ok"}"#.to_string())
}

async fn delete_trash(
    Path(id): Path<i64>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let state = state.lock().await;
    let db_url = state.db_url.as_ref().ok_or("NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .execute("DELETE FROM trash WHERE id = $1", &[&id])
        .await
        .map_err(|e| e.to_string())?;

    Ok(r#"{"status": "deleted"}"#.to_string())
}

async fn get_logs(State(state): State<Arc<Mutex<AppState>>>) -> Result<Json<Vec<LogItem>>, String> {
    let state = state.lock().await;
    let db_url = state.db_url.as_ref().ok_or("NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query(
            "SELECT id, level, message, created_at FROM logs ORDER BY created_at DESC LIMIT 100",
            &[],
        )
        .await
        .map_err(|e| e.to_string())?;

    let items: Vec<LogItem> = rows
        .iter()
        .map(|r| LogItem {
            id: r.get(0),
            level: r.get(1),
            message: r.get(2),
            created_at: r.get(3),
        })
        .collect();

    Ok(Json(items))
}

#[derive(Deserialize)]
struct LogInput {
    level: Option<String>,
    message: String,
    created_at: Option<f64>,
}

async fn add_log(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LogInput>,
) -> Result<String, String> {
    let state = state.lock().await;
    let db_url = state.db_url.as_ref().ok_or("NEON_DB_URL not set")?;

    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await.map_err(|e| e.to_string())?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let level = payload.level.unwrap_or_else(|| "info".to_string());
    let message = payload.message;
    let created_at = payload.created_at.unwrap_or_else(|| {
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

    Ok(r#"{"status": "ok"}"#.to_string())
}

#[tokio::main]
async fn main() {
    let worker_url = std::env::var("NULLITH_URL")
        .unwrap_or_else(|_| "https://nullith-worker.shedrackgodstime.workers.dev".to_string());
    let api_key = std::env::var("NULLITH_API_KEY").expect("NULLITH_API_KEY not set");
    let db_url = std::env::var("NEON_DB_URL").ok();

    let state = Arc::new(Mutex::new(AppState {
        worker_url,
        api_key,
        db_url,
        client: Client::new(),
    }));

    let app = Router::new()
        .route("/", get(|| async { "Nullith API\n" }))
        .route("/notes", get(get_notes))
        .route("/notes/:key", get(get_note))
        .route("/files", get(list_files))
        .route("/files/:path", get(get_file))
        .route("/trash", get(list_trash).post(add_trash))
        .route("/trash/:id", delete(delete_trash))
        .route("/logs", get(get_logs).post(add_log))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}