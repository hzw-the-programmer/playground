use tracing::{field, info, instrument};

#[derive(Debug)]
struct RobotState {
    mode: &'static str,
}

#[instrument]
fn report_state(state: RobotState) {
    // 用 field::debug 以 Debug 格式打印
    info!(state = field::debug(&state), "当前机器人状态");
    let _ = state.mode;
}

fn main() {
    tracing_subscriber::fmt::init();
    let s = RobotState { mode: "auto" };
    report_state(s);
}
