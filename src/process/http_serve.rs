use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug, serde::Deserialize)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("Serving {:?} on port {}", path, port);
    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .route("/*key", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file  {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("File not found"))
    } else {
        //todo test p is a directory
        //if it is a directory, list all files/subdirectories
        //as <li><a href="path/to/file">file</a></li>
        let content = match tokio::fs::read_to_string(&p).await {
            Ok(content) => content,
            Err(e) => {
                warn!("Error reading file {:?} : {:?}", p.display(), e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                );
            }
        };
        (StatusCode::OK, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let (status, content) = file_handler(State(state), path).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.contains("[package]"));
    }

    #[tokio::test]
    async fn test_file_handler_not_found() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("not_found.txt".to_string());
        let (status, content) = file_handler(State(state), path).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(content, "File not found");
    }
}
