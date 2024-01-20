use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rspc::Config;
pub use rspc::RouterBuilder;
use serde::Deserialize;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;

pub type PublicRouter = rspc::Router<Api>;

use crate::{router::todo_router::todo_router, service::task_service::TaskService};
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

fn api_router() -> rspc::Router<Api> {
    PublicRouter::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(".")
                    .join("web//src/utils/api.ts"),
            ),
        )
        .merge("todo.", todo_router())
        .build()
}

#[derive(Clone, Debug)]
pub struct Api {
    pub task_service: Arc<TaskService>,
}

pub fn new(cors: CorsLayer, task_service: Arc<TaskService>) -> axum::Router {
    let rspc = api_router().arced();

    Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc.endpoint(|| Api { task_service }).axum())
        .layer(cors)
}
