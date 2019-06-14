extern crate redis;

mod lib;
use lib::ws_redis::{init_redis_client, init_redis_connection, init_redis_subscriptions};
use lib::websocket::create_websocket_server;
use lib::config::Config;
use redis::{Client, Connection};

fn main () {
    let config = Config {
        channels: vec!["test1".to_string(), "test2".to_string()],
        redis_path: "redis://127.0.0.1/".to_string(),
        ws_path: "127.0.0.1:3012".to_string(),
    };
    let redis_client: Client = init_redis_client();
    let mut redis_connection: Connection = init_redis_connection(&redis_client);
    init_redis_subscriptions(&mut redis_connection, &config);
    create_websocket_server(&redis_client, &config);
}