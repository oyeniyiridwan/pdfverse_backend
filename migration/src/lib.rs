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
