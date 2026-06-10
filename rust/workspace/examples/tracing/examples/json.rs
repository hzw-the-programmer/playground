//! {
//!     "timestamp":"2026-06-10T10:35:12.732510Z",
//!     "level":"INFO",
//!     "fields":{
//!         "message":"机器人移动",
//!         "direction":"forward",
//!         "speed":0.8
//!     },
//!     "target":"json",
//!     "span":{
//!         "vel":0.8,
//!         "name":"robot_move"
//      },
//!     "spans":[{
//!         "vel":0.8,
//!         "name":"robot_move"
//!     }]
//! }

use tracing::{info, instrument};

#[instrument]
fn robot_move(vel: f64) {
    info!(direction = "forward", speed = vel, "机器人移动");
}

fn main() {
    tracing_subscriber::fmt()
        .json() // 开启 JSON 格式
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    robot_move(0.8);
}
