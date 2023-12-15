use serde::Deserialize;
use axum::extract::Json;
use axum::extract::State;
use axum::extract::Path as AxPath;
use axum::debug_handler;
use sqlx::PgPool as SqlxPgPool;

use crate::service::task;




#[derive(Debug, Deserialize)]
pub struct Task
{
    pub id: i32,

    pub note: String,
    pub done: bool,
}


#[derive(Debug, Deserialize)]
pub struct NewTask
{
    pub note: String,

}


#[derive(Debug, Deserialize)]
pub struct  UpdateTask
{
    pub note: String,
    pub done: bool,
}


#[debug_handler]
pub async fn create(pool: State<SqlxPgPool>, task: Json<Task>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

    sqlx::query!(r#"
        INSERT INTO
            tasks ("note", "done")
        VALUES
            ($1, $2)
        "#, task.note, task.done)
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Task Create")
}


#[debug_handler]
pub async fn select(pool: State<SqlxPgPool>, task:AxPath<i32>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

    let task = sqlx::query_as!(Task, r#"
        SELECT * FROM tasks
        WHERE id = $1
        "#, task.id )
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Selected Task")
}


#[debug_handler]
pub async fn all_tasks(pool: State<SqlxPgPool>, Json(task): Json<task::Task>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task;

    let task = sqlx::query_as!(Task, r#"
        SELECT * FROM tasks
        "#, task)
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("All Tasks")

}


#[debug_handler]
pub async fn update(pool: State<SqlxPgPool>, AxPath(id): AxPath<i32>, Json(task): Json<task::UpdateTask>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.note;

    let task = sqlx::query_as!(Task, r#"
        SELECT
            *
        FROM
            tasks
        WHERE
            id= $1
        "#, task.id, task.note, task.done )
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Update Task")

}


#[debug_handler]
pub async fn delete(pool: State<SqlxPgPool>, task: AxPath<i32>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

        let task = sqlx::query!(r#"
        DELETE FROM tasks.t
        WHERE id = $1
        "#, task )
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Task Deleted")


}