//! RUST_LOG=info cargo run --example robot
//! 2026-06-10T11:28:20.866285Z  INFO robot: ===== 机器人自主巡航启动 =====
//! 2026-06-10T11:28:20.866454Z  INFO obstacle_decision{dist=45.2}: robot: 距离正常，正常行驶
//! 2026-06-10T11:28:20.866502Z  INFO robot: 单次巡航周期结束
//!
//! RUST_LOG=debug cargo run --example robot
//! 2026-06-10T11:27:12.966735Z  INFO robot: ===== 机器人自主巡航启动 =====
//! 2026-06-10T11:27:12.966985Z DEBUG read_ultrasonic: robot: 读取超声波距离 distance=45.2
//! 2026-06-10T11:27:12.967080Z  INFO obstacle_decision{dist=45.2}: robot: 距离正常，正常行驶
//! 2026-06-10T11:27:12.967123Z DEBUG set_motor_speed: robot: 设置电机转速 target_speed=0.5
//! 2026-06-10T11:27:12.967155Z  INFO robot: 单次巡航周期结束

use tokio;
use tracing::{debug, error, info, instrument, warn};

/// 模拟超声波传感器读取
#[instrument(level = "debug", skip_all)]
async fn read_ultrasonic() -> f64 {
    let dist = 45.2;
    debug!(distance = dist, "读取超声波距离");
    dist
}

/// 避障决策
#[instrument]
async fn obstacle_decision(dist: f64) -> f64 {
    if dist < 20.0 {
        warn!(min_dist = 20.0, current = dist, "距离过近，减速");
        0.1
    } else {
        info!("距离正常，正常行驶");
        0.5
    }
}

/// 电机控制
#[instrument(skip(speed))]
async fn set_motor_speed(speed: f64) {
    if speed < 0.0 {
        error!("速度非法");
        return;
    }
    debug!(target_speed = speed, "设置电机转速");
}

#[tokio::main]
async fn main() {
    // 初始化：控制台 + 环境变量过滤
    // tracing_subscriber::fmt().init();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    // tracing_subscriber::fmt::init();

    info!("===== 机器人自主巡航启动 =====");

    let dist = read_ultrasonic().await;
    let speed = obstacle_decision(dist).await;
    set_motor_speed(speed).await;

    info!("单次巡航周期结束");
}
