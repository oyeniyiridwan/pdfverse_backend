use std:: time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn database_connection(database_url: &str)->Result<DatabaseConnection, DbErr>{
    let mut opt = ConnectOptions::new(database_url);
opt.max_connections(20)       // handle more concurrent queries
   .min_connections(5)        // keep some connections alive
   .connect_timeout(Duration::from_secs(5))
   .idle_timeout(Duration::from_secs(30));
 Database::connect(opt).await

}