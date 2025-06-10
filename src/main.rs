
use std::process::Command;
use std::net::TcpListener;
use std::io::{Read, Write};
use urlencoding::decode;
use serde::Serialize;
use chrono::Local;


#[derive(Serialize)]
enum State {
    Up,
    Down,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Up => write!(f, "Up"),
            State::Down => write!(f, "Down"),
        }
    }
}


#[derive(Serialize)]
struct Liveness {
    date: String,
    target: String,
    state: String,
    message: String,
} 

fn main() {

    let now = Local::now().format("%Y-%m-%d %H:%M:%S");

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("{} Info: Listening on 0.0.0.0:8080", now);

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 1024];
            let _ = stream.read(&mut buffer);

            let request = String::from_utf8_lossy(&buffer);
            let raw: &str  = request.split("ip=").nth(1)
                                    .and_then(|s| s.split_whitespace().next())
                                    .unwrap_or("");
                                
            let ip = decode(raw).unwrap();

            let output = Command::new("bash")
                .arg("-c")
                .arg(format!("ping -c 3 {}", ip.to_string()))
                .output()
                .expect("failed to execute");

            println!("{} Info: received request to ping: {}",now, ip);

            let liveness = Liveness {
                date: now.to_string(),
                target: ip.to_string(),
                state: if output.status.success() { 
                    State::Up 
                } else { 
                    State::Down 
                }.to_string(),
                message: String::from_utf8_lossy(&output.stdout).to_string(),
            };

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/json\r\n\r\n{}",
                serde_json::to_string(&liveness).unwrap()
            );
            let _ = stream.write(response.as_bytes());
            let _ = stream.flush();
        }
    }
}

