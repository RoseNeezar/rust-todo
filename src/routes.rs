use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use axum::{
    extract::State,
    response::Result,
    routing::{get, post},
    Json, Router,
};
pub use rspc::RouterBuilder;
use rspc::{Config, Error, ErrorCode};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Ctx {}
pub type PublicRouter = rspc::Router<Ctx>;

use crate::{repository::task_repository::Task, service::task_service::TaskService};

pub(crate) fn what() -> RouterBuilder<Ctx> {
    PublicRouter::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(".")
                    .join("web//src/utils/api.ts"),
            ),
        )
        .query("omega=what", |t| t(|_, _: ()| Ok("ok===what=again")))
}

#[derive(Clone)]
pub struct Api {
    task_service: Arc<TaskService>,
}

async fn api_login_handler(
    State(mm): State<Api>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Task>, String> {
    let new_todo = mm.task_service.create_todo(payload.title).await;
    dbg!("{:?}", &new_todo);

    match new_todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => match e {
            _ => Err(e.to_string()),
        },
    }
}

async fn api_get_todo_handler(
    State(mm): State<Api>,
    Json(payload): Json<GetTodoPayload>,
) -> Result<Json<Task>, String> {
    let todo = mm.task_service.get_todo(payload.id).await;
    dbg!("{:?}", &todo);

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => match e {
            _ => Err(e.to_string()),
        },
    }
}

pub fn new(task_service: Arc<TaskService>) -> axum::Router {
    let api = Api { task_service };

    let rspc = what().build().arced();

    Router::new()
        .route("/api/test", post(api_login_handler))
        .route("/api/get", get(api_get_todo_handler))
        .nest("/rspc", rspc.endpoint(|| Ctx {}).axum())
        .with_state(api)
}

#[derive(Debug, Deserialize)]
struct GetTodoPayload {
    id: String,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    title: String,
}
