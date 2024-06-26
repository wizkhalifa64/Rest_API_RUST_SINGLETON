use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{model::task_model::TaskInput, SingletonContext};

pub async fn create_task(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<TaskInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut data = state.write().await;
    data.task_data.add(TaskInput {
        title: body.title,
        user_id,
        done: false,
    });
    Ok((StatusCode::OK, Json(json!("Success"))))
}

pub async fn get_all_tasks_by_user_id(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = state.read().await;
    println!("{:?}", data);
    let task_list = data.task_data.get_all_tasks_by_user_id(user_id);
    Ok((StatusCode::OK, Json(json!({"task":task_list}))))
}

pub async fn get_specific_tasks_by_user_id(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    Path((user_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = state.read().await;
    let task_list = data
        .task_data
        .get_specific_tasks_by_user_id(user_id, task_id);
    Ok((StatusCode::OK, Json(json!({"task":task_list}))))
}

pub async fn update_specific_tasks_by_user_id(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    Path((user_id, task_id)): Path<(Uuid, Uuid)>,
    Json(task): Json<TaskInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut data = state.write().await;
    let response = data
        .task_data
        .update_specific_tasks_by_user_id(user_id, task_id, task);
    match response {
        Some(res) => Ok((StatusCode::OK, Json(json!({"task_id":res})))),
        None => Ok({
            let error_response = serde_json::json!({
                "message": "Not found".to_string()
            });
            (StatusCode::NOT_FOUND, Json(error_response))
        }),
    }
}
pub async fn delete_specific_tasks_by_user_id(
    State(state): State<Arc<RwLock<SingletonContext>>>,
    Path((user_id, task_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut data = state.write().await;
    let response = data.task_data.remove_specific_task_by_id(user_id, task_id);
    match response {
        Some(res) => Ok((StatusCode::OK, Json(json!({"task_id":res})))),
        None => Ok({
            let error_response = serde_json::json!({
                "message": "Not found".to_string()
            });
            (StatusCode::NOT_FOUND, Json(error_response))
        }),
    }
}
