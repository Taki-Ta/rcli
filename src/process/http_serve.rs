use crate::get_file_content;
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
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
) -> (StatusCode, Response) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file  {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            "File not found".to_string().into_response(),
        )
    } else if p.is_dir() {
        match std::fs::read_dir(p.clone()) {
            Ok(entries) => {
                let tmpl = get_file_content("fixtures/tmpl.html").unwrap();
                let tmpl = String::from_utf8(tmpl).unwrap();
                let mut content = String::new();
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let name = path.file_name().unwrap().to_str().unwrap();
                    content.push_str(&format!(
                        r#"<li><a href="/tower/{}">{}</a></li>"#,
                        //get the relative path
                        path.strip_prefix(&state.path).unwrap().to_string_lossy(),
                        name
                    ));
                }
                let res = Html(tmpl.replace("{{content}}", &content));
                (StatusCode::OK, res.into_response())
            }
            Err(e) => {
                warn!("Error reading dir {:?} : {:?}", p.display(), e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading dir".into_response(),
                )
            }
        }
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => (StatusCode::OK, Json(content).into_response()),
            Err(e) => {
                warn!("Error reading file {:?} : {:?}", p.display(), e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".into_response(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let (status, content) = file_handler(State(state), path).await;
        assert_eq!(status, StatusCode::OK);
        let bytes = to_bytes(content.into_body(), usize::MAX).await.unwrap();
        let body_string = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body_string.contains("[package]"));
    }

    #[tokio::test]
    async fn test_file_handler_not_found() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let path = Path("not_found.txt".to_string());
        let (status, content) = file_handler(State(state), path).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        let bytes = to_bytes(content.into_body(), usize::MAX).await.unwrap();
        let body_string = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body_string, "File not found".to_string());
    }
}
