use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("192.168.0.10:35000").await.unwrap();

    let version = send_command(&mut stream, "ATZ").await;      // Reset and get version
    println!("ELM327 Version: {}", version.trim());

    send_command(&mut stream, "ATE0").await;     // Echo off
    send_command(&mut stream, "ATSP0").await;    // Auto-select protocol
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Wait for protocol detection

    let voltage = send_command(&mut stream, "ATRV").await;     // Get voltage
    println!("Voltage: {}", voltage.trim());

}

async fn send_command(stream: &mut TcpStream, command: &str) -> String {
    let cmd = format!("{}\r", command); // Commands to ELM327 are terminated with a carriage return
    stream.write_all(cmd.as_bytes()).await.unwrap();

    let mut buffer = vec![0u8; 128];  // Adjust buffer size as needed
    let _ = stream.read(&mut buffer).await.unwrap();
    let response = String::from_utf8_lossy(&buffer).into_owned();
    response
}
