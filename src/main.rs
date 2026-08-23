use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let target = "127.0.0.1";
    println!("Scanning ports on target: {}", target);
    for port in 79..=85 {
        let addr = format!("{}:{}", target, port);
        match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(500)) {
            Ok(_) => println!("Port {} is OPEN", port),
            Err(_) => println!("Port {} is closed", port),
        }
    }
}
