use redis::{RedisError, aio::MultiplexedConnection};

pub async fn redis_database_connection(redis_url: &str)->Result<MultiplexedConnection, RedisError>{
 let redis_client = redis::Client::open(redis_url)?;
redis_client
        .get_multiplexed_async_connection()
        .await
}


