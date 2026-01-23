use axum::{Json, extract::FromRequest};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::{domain::task::Task, utils::error::ApiError};

#[derive(Serialize, Deserialize, Debug)]

pub struct RequestTaskFilter {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug)]

pub struct RequestUpdateTask {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
   
    pub is_default: Option<bool>
}




#[derive(Serialize, Deserialize,Validate, Debug)]
pub struct RequestTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    #[validate(length(min =3))]
    pub title: String,
    pub description: Option<String>,
    pub is_default: Option<bool>,
}

impl<S> FromRequest<S> for RequestTask
where 
S: Send + Sync
 {
    type Rejection = ApiError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) ->  Result<Self, Self::Rejection>  {
        let Json(task) = Json::<RequestTask>::from_request(req, state)
        .await.map_err(|e| ApiError::BadRequest(format!("{e}")))?;
    if let  Err(e) = task.validate() {
        return Err(ApiError::BadRequest(format!("{e}")));
    }
       Ok(task)
       
    }
}




#[skip_serializing_none]
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct TaskResponse{
  id: i32,
    priority: Option<String>,
    title:String,
    description: Option<String>,
    user_id: String,


}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
       Self { id: task.id.expect("task id not supplied by domain Task"), priority: task.priority, title: task.title, description: task.description, user_id: task.user_id.to_string() }
    }
}







#[derive(Serialize, Deserialize)]
pub struct TaskDeleteParam {
   pub soft: bool,
}
