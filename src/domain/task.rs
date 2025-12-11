
use uuid::Uuid;

use crate::database::tasks::Model;

pub struct Task {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub user_id: Uuid
}

impl Task {
    pub fn new(priority: Option<String>, title:String, description:Option<String>,user_id: Uuid)->Self{
Self { id: None, 
    priority: priority
    , title:title, 
    user_id:user_id,
    description:description }
    }
}

impl From<Model> for Task {
    fn from(model: Model) -> Self {
        Self { 
            id: Some(model.id),
            priority:model.priority,
             title: model.title,
             user_id:model.user_id,
             description: model.description}
    }
}