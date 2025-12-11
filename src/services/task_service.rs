use std::sync::Arc;
use uuid::Uuid;
use crate::{domain::task::Task, dtos::task_dto::{RequestTaskFilter, RequestUpdateTask}, repository::task_repository::TaskRepository, utils::error::ApiError};

pub trait TaskService{
    async fn create_task(&self,priority: Option<String>, title:String, description:Option<String>,user_id: Uuid)->Result<Task,ApiError>;
    async fn delete_task(&self,soft:bool, id:i32, user_id: Uuid) -> Result<String,ApiError>;

    async fn get_one_task(&self,id:i32, user_id: Uuid)->Result<Task,ApiError>;
     async fn get_all_task(&self, user_id: Uuid,filter:RequestTaskFilter)-> Result<Vec<Task>,ApiError>;
         async fn complete_update_task(&self,priority: Option<String>, title:String, description:Option<String>,user_id: Uuid, id:i32)->Result<Task,ApiError>;

    async fn partial_update_task(&self, user_id: Uuid, id:i32, update_task:RequestUpdateTask)->Result<Task,ApiError>;
}


pub struct TaskServiceImpl<T: TaskRepository + Send + Sync>{
    pub task_repo: Arc<T>
}

impl<T: TaskRepository + Send + Sync> TaskServiceImpl<T> {
    pub fn new(task_repo: Arc<T>) -> Self {
        Self { task_repo }
    }
}

impl<T> TaskService for TaskServiceImpl<T>
where 
 T: TaskRepository + Send + Sync  {
    async fn create_task(&self,priority: Option<String>, title:String, description:Option<String>,user_id: Uuid)->Result<Task,ApiError> {
        let task = Task::new(priority, title, description, user_id);
       self.task_repo.create_task(task).await

    }
    async fn complete_update_task(&self,priority: Option<String>, title:String, description:Option<String>,user_id: Uuid, id:i32)->Result<Task,ApiError> {
        let task = Task::new(priority, title, description, user_id);
       self.task_repo.complete_update_task(id,user_id,task).await

    }

    async fn partial_update_task(&self,user_id: Uuid, id:i32, update_task:RequestUpdateTask)->Result<Task,ApiError> {
       self.task_repo.partial_update_task(id,user_id, update_task).await

    }


        async fn delete_task(&self,soft:bool, id:i32, user_id: Uuid) -> Result<String,ApiError> {
        self.task_repo.delete_task_from_repo(soft, id, user_id).await
        }

            async fn get_one_task(&self,id:i32, user_id: Uuid)->Result<Task,ApiError>{
          match   self.task_repo.find_task_by_id( id, user_id).await?{
            Some(task) =>Ok(task),
            None =>{
                return Err( ApiError::NotFound("Task not found".to_string()));
            }
                  }

            }
                 async fn get_all_task(&self, user_id: Uuid,filter:RequestTaskFilter)-> Result<Vec<Task>,ApiError>{
                    self.task_repo.find_user_tasks(user_id, filter).await
                 }


 }

