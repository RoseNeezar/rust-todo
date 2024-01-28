use rspc::{Error, ErrorCode, RouterBuilder, Type};
use serde::{Deserialize, Serialize};

use crate::{
    errors::ErrorResponse,
    repository::task_repository::TaskStatus,
    routes::{Api, PublicRouter},
};

pub fn todo_router() -> RouterBuilder<Api> {
    PublicRouter::new()
        .query("get", |t| {
            t(|ctx, todo_id: i32| async move {
                match ctx.task_service.get_todo(todo_id).await {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest.to_string(),
                            )),
                            ErrorResponse::NoTodo => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo.to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();

                            Err(Error::new(ErrorCode::InternalServerError, error_message))
                        }
                    },
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
                match ctx
                    .task_service
                    .create_todo(input.title, input.status)
                    .await
                {
                    Ok(data) => Ok(data),
                    Err(e) => match e.downcast_ref::<ErrorResponse>() {
                        Some(err) => match err {
                            ErrorResponse::InvalidRequest => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest.to_string(),
                            )),
                            ErrorResponse::NoTodo => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo.to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();

                            Err(Error::new(ErrorCode::InternalServerError, error_message))
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
                            ErrorResponse::InvalidRequest => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest.to_string(),
                            )),
                            ErrorResponse::NoTodo => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo.to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();

                            Err(Error::new(ErrorCode::InternalServerError, error_message))
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
                            ErrorResponse::InvalidRequest => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::InvalidRequest.to_string(),
                            )),
                            ErrorResponse::NoTodo => Err(Error::new(
                                ErrorCode::BadRequest,
                                ErrorResponse::NoTodo.to_string(),
                            )),
                        },
                        None => {
                            let error_message = e.to_string();

                            Err(Error::new(ErrorCode::InternalServerError, error_message))
                        }
                    },
                }
            })
        })
}
