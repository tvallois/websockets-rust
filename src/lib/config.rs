#[derive(Clone)]
pub struct Config {
    pub channels: Vec<String>,
    pub redis_path: String,
    pub ws_path: String,
}