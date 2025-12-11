use axum::{Extension, Json, extract::{Path, Query,State}};
use uuid::Uuid;
use crate::{app::state::ConcreteAppState, custom_middleware::Claim, dtos::task_dto::{RequestTask, RequestTaskFilter, RequestUpdateTask, TaskDeleteParam, TaskResponse}, services::task_service::TaskService, utils::error::ApiError};





pub async fn create_task(
    Extension(claim):Extension<Claim>,
 State(state):State<ConcreteAppState>,
      task:RequestTask)-> Result<Json<TaskResponse>,ApiError>{
           let user_id =claim.sub;
    let task = state.task_service.create_task(
        task.priority, task.title, task.description, user_id).await? ;
        Ok(Json(TaskResponse::from(task)))   

}





pub async fn combine_atomic_soft_delete_task(
        Extension(claim):Extension<Claim>,

 State(state):State<ConcreteAppState>,
    Path(id): Path<i32>,
    Query(delete_params): Query<TaskDeleteParam>,
) -> Result<String, ApiError> {
           let user_id =claim.sub;

    state.task_service.delete_task(delete_params.soft, id, user_id).await
   
}






pub async fn get_one_task(
        Extension(claim):Extension<Claim>,

 State(state):State<ConcreteAppState>,
    Path(id): Path<i32>,
) -> Result<Json<TaskResponse>,ApiError> {
           let user_id =claim.sub;

    let task = state.task_service.get_one_task(id, user_id).await?;
        Ok(Json(TaskResponse::from(task)))   

}



pub async fn get_all_task(
        Extension(claim):Extension<Claim>,

 State(state):State<ConcreteAppState>,
    Query(task_filter): Query<RequestTaskFilter>,
) -> Result<Json<Vec<TaskResponse>>,ApiError> {
           let user_id =claim.sub;
let tasks = state.task_service.get_all_task(user_id,task_filter).await?;

  let tasks_response = 
  tasks.into_iter().map(|t|TaskResponse::from(t) ).collect();

    Ok(Json(tasks_response))
}




pub async fn atomic_update(
        Extension(claim):Extension<Claim>,

 State(state):State<ConcreteAppState>,
    Path(id): Path<i32>,
     task:RequestTask,
) -> Result<Json<TaskResponse>, ApiError> {
           let user_id =claim.sub;
let task =state.task_service.complete_update_task(task.priority, task.title, task.description, user_id, id).await?;
        Ok(Json(TaskResponse::from(task)))   

}






pub async fn partial_update(
        Extension(claim):Extension<Claim>,

    State(state):State<ConcreteAppState>,
    Path(id): Path<i32>,
     Json(update_task):Json<RequestUpdateTask>,
) -> Result<Json<TaskResponse>, ApiError> {
           let user_id =claim.sub;

    let task =state.task_service.partial_update_task(user_id, id, update_task).await?;
        Ok(Json(TaskResponse::from(task)))   

}

