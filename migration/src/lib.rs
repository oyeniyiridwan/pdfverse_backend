pub use sea_orm_migration::prelude::*;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_134147_add_users_tasks_providers::Migration),
            Box::new(m20251028_142356_add_users_tasks_providers::Migration),
            Box::new(m20251031_040732_add_users_tasks_providers::Migration),
            Box::new(m20251103_144847_add_users_tasks_providers::Migration),
            Box::new(m20251109_003415_add_users_tasks_providers::Migration),
            Box::new(m20251109_012147_add_users_tasks_providers::Migration),
            Box::new(m20251109_134418_add_users_tasks_providers::Migration),
            Box::new(m20251109_142852_add_users_tasks_providers::Migration),
            Box::new(m20251110_024109_add_users_tasks_providers::Migration),
            Box::new(m20251112_044056_add_users_tasks_providers::Migration),
            Box::new(m20251112_085253_add_users_tasks_providers::Migration),
            Box::new(m20251124_150954_add_users_tasks_providers::Migration),
            Box::new(m20251215_003349_chat_info_and_chat_message::Migration),
            Box::new(m20251215_020408_chat_info_and_chat_message::Migration),
            Box::new(m20251217_124510_document_chunk::Migration),
            Box::new(m20260121_213450_update_time_document::Migration),
            Box::new(m20260122_022145_update_message_timestamp::Migration),
        ]
    }
}
mod m20251028_134147_add_users_tasks_providers;
mod m20251028_142356_add_users_tasks_providers;
mod m20251031_040732_add_users_tasks_providers;
mod m20251103_144847_add_users_tasks_providers;
mod m20251109_003415_add_users_tasks_providers;
mod m20251109_012147_add_users_tasks_providers;
mod m20251109_134418_add_users_tasks_providers;
mod m20251109_142852_add_users_tasks_providers;
mod m20251110_024109_add_users_tasks_providers;
mod m20251112_044056_add_users_tasks_providers;
mod m20251112_085253_add_users_tasks_providers;
mod m20251124_150954_add_users_tasks_providers;
mod m20251215_003349_chat_info_and_chat_message;
mod m20251215_020408_chat_info_and_chat_message;
mod m20251217_124510_document_chunk;
mod m20260121_213450_update_time_document;
mod m20260122_022145_update_message_timestamp;
