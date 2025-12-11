
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, prelude::DateTimeWithTimeZone};
use uuid::Uuid;

use crate::{database::tasks::{ActiveModel, Column, Entity as Tasks, Model}, domain::task::Task, dtos::task_dto::{ RequestTaskFilter, RequestUpdateTask}, utils::error::ApiError};

pub trait TaskRepository: Send + Sync{
    async fn create_task(&self, task:Task )->Result<Task,ApiError>;
    async fn delete_task_from_repo(&self,soft:bool, id:i32, user_id: Uuid) -> Result<String,ApiError>;
    async fn find_task_by_id(&self, id:i32, user_id: Uuid)->Result<Option<Task>,ApiError>;
        async fn find_user_tasks(&self, user_id: Uuid,filter:RequestTaskFilter)->Result<Vec<Task>,ApiError>;
        async fn partial_update_task(&self,id:i32,user_id:Uuid,new_task: RequestUpdateTask) -> Result<Task,ApiError>;
          async fn complete_update_task(&self,id:i32,user_id:Uuid,new_task: Task) -> Result<Task,ApiError>;

}


pub struct TaskRepositoryImpl{
    db: DatabaseConnection
}

 fn basic_user_filter(user_id: Uuid)->Condition{
    Condition::all().add(Column::UserId.eq(user_id))
    }

impl TaskRepositoryImpl {
    pub fn new(db :DatabaseConnection)->Self{
        Self { db }
    }

 

fn _user_task_filter(&self,task_filter: &RequestTaskFilter, user_id: Uuid) -> Condition {
    let mut condition = basic_user_filter(user_id);


    if let Some(ref title) = task_filter.title {
        condition = condition.add(Column::Title.contains(title));
    }

    if let Some(id) = task_filter.id {
        condition = condition.add(Column::Id.eq(id));
    }

    if let Some(ref description) = task_filter.description {
       condition = if description.is_empty() {
             condition.add(Column::Description.is_null())
        } else {
            condition.add(Column::Description.contains(description))
        }
    }


    if let Some(ref priority) = task_filter.priority {
     condition =    if priority.is_empty() {
            condition.add(Column::Priority.is_null())
        } else {
            condition.add(Column::Priority.eq(priority))
        }
    }

    condition
}



async fn find_task_model_by_id(&self, id:i32, user_id: Uuid)->Result<Option<Model>,ApiError>{
    let mut condition =basic_user_filter(user_id);
       condition = condition.add(Column::Id.eq(id));
    
         Tasks::find().filter(condition).one(&self.db).await
.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )
}



}

impl TaskRepository for TaskRepositoryImpl
 {
    async fn create_task(&self, task:Task )->Result<Task,ApiError> {
          let active_task = ActiveModel{
            priority: Set(task.priority),
            title: Set(task.title),
            description: Set(task.description),
            user_id: Set( task.user_id),
            ..Default::default()
        };
    let model =    active_task.insert(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
    Ok(Task::from(model))

    }

    async fn delete_task_from_repo(&self,soft:bool, id:i32, user_id: Uuid) -> Result<String,ApiError> {
        let mut condition =basic_user_filter(user_id);
       condition = condition.add(Column::Id.eq(id));
        let task = Tasks::find()
        .filter(condition).one(&self.db).await.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
 let task_model =      match task{
        Some(model) => model,
        None => {
         return   Err(ApiError::NotFound(format!("task with id: {} does not exist",id)));
        },
           };
match soft{
    true =>{
        match task_model.deleted_at {
            Some(_) => {
               return   Err(ApiError::Forbidden(format!("task with id: {} already soft deleted",id)));

            },
            None => {
                let mut active_task = task_model.into_active_model();
                active_task.deleted_at = Set(Some(DateTimeWithTimeZone::from(Utc::now())));
                active_task.update(&self.db).await
.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
            },
        }
    },
    false => {
       let delete_result =  task_model.into_active_model()
       .delete(&self.db).await
.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
    if delete_result.rows_affected ==0{
               return   Err(ApiError::NotFound(format!("task with id: {}  not found",id)));

    }
    
    },
}
      Ok(format!("task with id: {id} deleted successfully"))


    }

    async fn find_task_by_id(&self, id:i32, user_id: Uuid)->Result<Option<Task>,ApiError> {

        let task = self.find_task_model_by_id(id, user_id).await?;
        Ok(task.map(Task::from))
    }

    async fn find_user_tasks(&self, user_id: Uuid,filter:RequestTaskFilter)->Result<Vec<Task>,ApiError> {
    let  mut condition =self._user_task_filter( &filter,user_id);
    condition = condition.add(Column::DeletedAt.is_null());
        let task = Tasks::find().filter(condition).all(&self.db).await
.map_err(|e|
    ApiError::internal_msg(format!("error: {e}"))
    )?;
    
   Ok(  task.into_iter().map(|t| Task::from(t)).collect())

        
    }
    
    async fn partial_update_task(&self,id:i32,user_id:Uuid,new_task: RequestUpdateTask) -> Result<Task,ApiError> {
        let mut task =  self.find_task_model_by_id(id, user_id).await?.ok_or_else(|| {ApiError::NotFound(format!("Task with id; {} not found",id))})?.into_active_model();
if let Some( priority) = new_task.priority{
    task.priority = Set(priority);
}
if let Some( description) = new_task.description{
    task.description = Set(description);
}

if let Some( title) = new_task.title{
    task.title = Set(title);
}

if let Some( is_default) = new_task.is_default{
    task.is_default = Set(is_default);
}
       
let task_model = task.update(&self.db).await.map_err(|e|
ApiError::internal_msg(format!("error:{e}")))?;
Ok(Task::from(task_model))

    }
    
    async fn complete_update_task(&self,id:i32,user_id:Uuid,new_task: Task) -> Result<Task,ApiError> {
   self.find_task_model_by_id(id, user_id).await?.ok_or_else(|| {ApiError::NotFound(format!("Task with id; {} not found",id))})?.into_active_model();
   let active_task = ActiveModel{
    id:Set(id),
            priority: Set(new_task.priority),
            title: Set(new_task.title),
            description: Set(new_task.description),
            user_id: Set( new_task.user_id),
            ..Default::default()
        };
        let task_model = active_task.update(&self.db).await.map_err(|e|
ApiError::internal_msg(format!("error:{e}")))?;
Ok(Task::from(task_model))
    }
}


