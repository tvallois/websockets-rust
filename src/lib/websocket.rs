extern crate ws;
extern crate redis;
extern crate serde;
extern crate serde_json;

use ws::{listen, Sender, Handler, Message, Result, CloseCode};
use redis::Client;
use serde::{Deserialize, Serialize};
use super::ws_redis::init_redis_connection;
use super::config::Config;
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct WSValue {
    key: String,
    user_id: u8,
    data: HashMap<String, Value>,
}

// Server WebSocket handler
struct Server {
    out: Sender,
    redis_client: Client,
    config: Config,
}

impl Server {
    fn ws_subscription(&self, ws_value: WSValue) {
        let redis_cn = init_redis_connection(&self.redis_client);
        if ws_value.data.contains_key("channels") {
            let channels: Vec<String> = serde_json::from_value(ws_value.data.get("channels").cloned().unwrap()).unwrap();
            let connection_id = String::from("connection_id_") + &self.out.connection_id().to_string();
            for channel in channels {
                let _ : () = redis::cmd("LPUSH").arg(&connection_id).arg(channel).query(&redis_cn).unwrap();
            }
            redis::cmd("HSET").arg("users").arg(&connection_id).arg(ws_value.user_id).execute(&redis_cn);
        }
    }
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        let ws_value: WSValue = match serde_json::from_str(msg.as_text().unwrap()) {
            Ok(ws_value) => ws_value,
            Err(error) => {
                println!("{}", error);
                return self.out.send("Incorrect format");
            }
        };
        if ws_value.key == String::from("subscribe") {
            self.ws_subscription(ws_value);
        }
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("{:?} {}", code, reason);
        let redis_cn = init_redis_connection(&self.redis_client);
        let connection_id = String::from("connection_id_") + &self.out.connection_id().to_string();
        redis::cmd("DEL").arg(&connection_id).execute(&redis_cn);
        redis::cmd("HDEL").arg("users").arg(&connection_id).execute(&redis_cn);
    }

}

pub fn create_websocket_server(redis_client: &Client, config: &Config) {
    let addr = &config.ws_path;
    listen(addr, |out| {
        Server { out: out, redis_client: redis_client.clone(), config: config.clone() }
    }).unwrap();
}

