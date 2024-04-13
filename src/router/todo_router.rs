use rspc::{Error, ErrorCode, RouterBuilder, Type};
use serde::{Deserialize, Serialize};

use crate::{
    errors::ErrorResponse,
    repository::task_repository::TaskStatus,
    routes::{PrivateRouter, UserCtx},
};

pub fn todo_router() -> RouterBuilder<UserCtx> {
    PrivateRouter::new()
        .query("get_all", |t| {
            t(|ctx, _: ()| async move {
                match ctx.task_service.get_all_todos().await {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest {
                                    error: err.to_string(),
                                }
                                .to_string(),
                            )),
                            ErrorResponse::NoTodo { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo {
                                    id: String::from("no todos"),
                                }
                                .to_string(),
                            )),
                        },
                        None => {
                            println!("change");

                            let error_message = e.to_string();
                            Err(Error::new(
                                ErrorCode::InternalServerError,
                                ErrorResponse::InvalidRequest {
                                    error: error_message,
                                }
                                .to_string(),
                            ))
                        }
                    },
                }
            })
        })
        .query("get", |t| {
            t(|ctx, todo_id: i32| async move {
                match ctx.task_service.get_todo(todo_id).await {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest {
                                    error: err.to_string(),
                                }
                                .to_string(),
                            )),
                            ErrorResponse::NoTodo { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo {
                                    id: todo_id.to_string(),
                                }
                                .to_string(),
                            )),
                        },
                        None => {
                            println!("change");

                            let error_message = e.to_string();
                            Err(Error::new(
                                ErrorCode::InternalServerError,
                                ErrorResponse::InvalidRequest {
                                    error: error_message,
                                }
                                .to_string(),
                            ))
                        }
                    },
                    // pnd pnc atomic transfer
                }
            })
        })
        .mutation("create", |t| {
            #[derive(Debug, Serialize, Deserialize, Type)]
            struct CreateTodoArgs {
                title: String,
                status: TaskStatus,
            }

            t(|ctx, input: CreateTodoArgs| async move {
                println!("change");

                match ctx
                    .task_service
                    .create_todo(input.title, input.status)
                    .await
                {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest {
                                    error: err.to_string(),
                                }
                                .to_string(),
                            )),
                            _ => {
                                let error_message = e.to_string();
                                Err(Error::new(
                                    ErrorCode::InternalServerError,
                                    ErrorResponse::InvalidRequest {
                                        error: error_message,
                                    }
                                    .to_string(),
                                ))
                            }
                        },
                        None => {
                            let error_messag = e.to_string();
                            Err(Error::new(
                                ErrorCode::InternalServerError,
                                ErrorResponse::InvalidRequest {
                                    error: error_messag,
                                }
                                .to_string(),
                            ))
                        }
                    },
                }
            })
        })
        .mutation("update", |t| {
            #[derive(Debug, Serialize, Deserialize, Type)]
            struct UpdateTodoArgs {
                id: i32,
                title: String,
                status: TaskStatus,
            }

            t(|ctx, input: UpdateTodoArgs| async move {
                match ctx
                    .task_service
                    .update_todo(input.id, Some(&input.title), Some(input.status))
                    .await
                {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest {
                                    error: err.to_string(),
                                }
                                .to_string(),
                            )),
                            ErrorResponse::NoTodo { .. } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo {
                                    id: input.id.to_string(),
                                }
                                .to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();
                            Err(Error::new(
                                ErrorCode::InternalServerError,
                                ErrorResponse::InvalidRequest {
                                    error: error_message,
                                }
                                .to_string(),
                            ))
                        }
                    },
                }
            })
        })
        .mutation("delete", |t| {
            #[derive(Debug, Serialize, Deserialize, Type)]
            struct DeleteTodoArgs {
                id: i32,
            }

            t(|ctx, input: DeleteTodoArgs| async move {
                match ctx.task_service.delete_todo(input.id).await {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest { error: _ } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest {
                                    error: err.to_string(),
                                }
                                .to_string(),
                            )),
                            ErrorResponse::NoTodo { id: _ } => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo {
                                    id: input.id.to_string(),
                                }
                                .to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();
                            Err(Error::new(
                                ErrorCode::InternalServerError,
                                ErrorResponse::InvalidRequest {
                                    error: error_message,
                                }
                                .to_string(),
                            ))
                        }
                    },
                }
            })
        })
}
