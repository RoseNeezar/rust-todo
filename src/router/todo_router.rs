use rspc::{Error, ErrorCode, RouterBuilder};

use crate::routes::{Api, PublicRouter};

pub fn todo_router() -> RouterBuilder<Api> {
    PublicRouter::new().query("get", |t| {
        t(|ctx, todo_id: i32| async move {
            let todo =
                ctx.task_service.get_todo(todo_id).await.map_err(|err| {
                    Error::with_cause(ErrorCode::BadRequest, "no todo".into(), err)
                })?;
            dbg!("{:?}", &todo);

            Ok(todo)
        })
    })
}
