fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;
    use std::time::Duration;
    use redis::{AsyncCommands, Client, Commands, RedisError};
    use redis::aio::MultiplexedConnection;

    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _ : () = con.del("names").await?;
        let _ : () = con.sadd("names", "Eko").await?;
        let _ : () = con.sadd("names", "Eko").await?;
        let _ : () = con.sadd("names", "Kurniawan").await?;
        let _ : () = con.sadd("names", "Kurniawan").await?;
        let _ : () = con.sadd("names", "Khannedy").await?;
        let _ : () = con.sadd("names", "Khannedy").await?;

        let len: i32 = con.scard("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = con.smembers("names").await?;
        assert_eq!(vec!["Eko", "Kurniawan", "Khannedy"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _ : () = con.del("names").await?;
        let _ : () = con.rpush("names", "Eko").await?;
        let _ : () = con.rpush("names", "Kurniawan").await?;
        let _ : () = con.rpush("names", "Khannedy").await?;

        let len: i32 = con.llen("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = con.lrange("names", 0, -1).await?;
        assert_eq!(vec!["Eko", "Kurniawan", "Khannedy"], names);

        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Eko"], names);
        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Kurniawan"], names);
        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Khannedy"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _ : () = con.set_ex("name", "Kurniawan", 2).await?;
        let value: String = con.get("name").await?;
        println!("{}", value);

        tokio::time::sleep(Duration::from_secs(5)).await;

        let value: Result<String, RedisError> = con.get("name").await;
        assert_eq!(true, value.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_async_connection() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _ : () = con.set("name", "Kurniawan").await?;
        let value: String = con.get("name").await?;

        println!("{}", value);

        Ok(())
    }

    async fn get_client() -> Result<MultiplexedConnection, RedisError> {
        let client = Client::open("redis://localhost:6379/")?;
        client.get_multiplexed_async_connection().await
    }

    #[test]
    fn test_connection() {
        let mut client = Client::open("redis://localhost:6379/").unwrap();

        let _: () = client.set("name", "Eko").unwrap();
        let value: String = client.get("name").unwrap();

        println!("{}", value);
    }
}