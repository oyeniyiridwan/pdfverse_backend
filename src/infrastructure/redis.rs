use redis::{RedisError, aio::MultiplexedConnection};




use redis::{Client, aio::MultiplexedConnection, RedisError};

pub async fn redis_database_connection(
    redis_url: &str
) -> Result<MultiplexedConnection, RedisError> {
    let client = Client::open(redis_url)?;

    let conn = client.get_multiplexed_tokio_connection().await?;

    Ok(conn)
}