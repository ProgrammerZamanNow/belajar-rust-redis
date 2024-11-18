fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::num::NonZero;
    use std::time::Duration;
    use redis::{AsyncCommands, Client, Commands, RedisError};
    use redis::aio::MultiplexedConnection;
    use redis::geo::{RadiusOptions, Unit};

    #[tokio::test]
    async fn test_pipeline() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        redis::pipe()
            .set_ex("name", "Eko", 2)
            .set_ex("address", "Indonesia", 2)
            .exec_async(&mut con).await?;

        let name: String = con.get("name").await?;
        assert_eq!("Eko", name);

        let address: String = con.get("address").await?;
        assert_eq!("Indonesia", address);

        Ok(())
    }

    #[tokio::test]
    async fn test_hyper_log_log() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("visitors").await?;
        let _: () = con.pfadd("visitors", ("eko", "kurniawan", "khannedy")).await?;
        let _: () = con.pfadd("visitors", ("eko", "budi", "joko")).await?;
        let _: () = con.pfadd("visitors", ("budi", "joko", "rully")).await?;

        let total: i32 = con.pfcount("visitors").await?;
        assert_eq!(6, total);

        Ok(())
    }

    #[tokio::test]
    async fn test_geo_point() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("sellers").await?;
        let _: () = con.geo_add("sellers", (106.822702, -6.177590, "Toko A")).await?;
        let _: () = con.geo_add("sellers", (106.820889, -6.174964, "Toko B")).await?;

        let distance: f64 = con.geo_dist("sellers", "Toko A", "Toko B", Unit::Kilometers).await?;
        assert_eq!(0.3543, distance);

        let result: Vec<String> = con.geo_radius("sellers", 106.821825, -6.175105, 0.5,
                                                 Unit::Kilometers, RadiusOptions::default()).await?;
        assert_eq!(vec!["Toko B", "Toko A"], result);

        Ok(())
    }

    #[tokio::test]
    async fn test_hash() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("user:1").await?;
        let _: () = con.hset("user:1", "id", "1").await?;
        let _: () = con.hset("user:1", "name", "Eko").await?;
        let _: () = con.hset("user:1", "email", "eko@gmail.com").await?;

        let user: HashMap<String, String> = con.hgetall("user:1").await?;
        assert_eq!("1", user.get("id").unwrap());
        assert_eq!("Eko", user.get("name").unwrap());
        assert_eq!("eko@gmail.com", user.get("email").unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_sorted_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.zadd("names", "Eko", 100).await?;
        let _: () = con.zadd("names", "Kurniawan", 85).await?;
        let _: () = con.zadd("names", "Khannedy", 95).await?;

        let len: i32 = con.zcard("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = con.zrange("names", 0, -1).await?;
        assert_eq!(vec!["Kurniawan", "Khannedy", "Eko"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.sadd("names", "Eko").await?;
        let _: () = con.sadd("names", "Eko").await?;
        let _: () = con.sadd("names", "Kurniawan").await?;
        let _: () = con.sadd("names", "Kurniawan").await?;
        let _: () = con.sadd("names", "Khannedy").await?;
        let _: () = con.sadd("names", "Khannedy").await?;

        let len: i32 = con.scard("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = con.smembers("names").await?;
        assert_eq!(vec!["Eko", "Kurniawan", "Khannedy"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.rpush("names", "Eko").await?;
        let _: () = con.rpush("names", "Kurniawan").await?;
        let _: () = con.rpush("names", "Khannedy").await?;

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
        let _: () = con.set_ex("name", "Kurniawan", 2).await?;
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
        let _: () = con.set("name", "Kurniawan").await?;
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