// Day06 — Enums & Pattern Matching

mod state_pattern;

#[derive(Debug)]
enum ConnectionState {
    Disconnected,
    Connecting(u32), // seconds
    Connected(String),
    Failed(String),
}

impl ConnectionState {
    fn describe(&self) -> String {
        match self {
            ConnectionState::Disconnected => "연결 끊김".to_string(),
            ConnectionState::Connecting(sec) => format!("연결 중... ({}초 경과)", sec),
            ConnectionState::Connected(addr) => format!("연결됨: {}", addr),
            ConnectionState::Failed(reason) => format!("실패: {}", reason),
        }
    }
}

fn simulate_connection(step: u32) -> ConnectionState {
    match step {
        0 => ConnectionState::Disconnected,
        1..=3 => ConnectionState::Connecting(step * 2),
        4 => ConnectionState::Connected("192.168.0.1". into()),
        _ => ConnectionState::Failed("Timeout".into()),
    }
}

fn main() {
    println!("=== [Day06] Enum & Pattern Matching ===");
    
    for step in 0..=5 {
        let state = simulate_connection(step);
        println!("[{:?}] {}", state, state.describe());
    }

    // Option & match 예시
    let maybe_value = Some(42);
    match maybe_value {
        Some(v) => println!("값 존재: {}", v),
        None => println!("값 없음"),
    }

    // Result & if let 예시
    let parsed: Result<i32, _> = "128".parse();
    if let Ok(num) = parsed {
        println!("문자열 파싱 성공: {}", num);
    } else {
        println!("파싱 실패");
    }

    println!("\n=== [State Pattern Demo] ===");
    state_pattern::run_demo();
}
