use redis::{Client, Connection};
use super::config::Config;

pub fn init_redis_client() -> Client {
    Client::open("redis://127.0.0.1/").unwrap()
}

pub fn init_redis_connection(client: &Client) -> Connection {
    let connexion = client.get_connection();
    return match connexion {
        Ok(redis_connexion) => redis_connexion,
        Err(error) => {
            panic!("Wrong path to redis or probably not installed. More info: {}", error);
        } 
    };
}

pub fn init_redis_subscriptions(connection: &mut Connection, config: &Config) {
    let mut pub_sub = connection.as_pubsub();
    for channel in &config.channels {
        pub_sub.subscribe(channel).unwrap();
    }
}
