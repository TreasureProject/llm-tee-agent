use std::io::{Read, Write};
use vsock_server::vsock::VsockClient;

fn main() -> std::io::Result<()> {
    let mut stream = VsockClient::connect(2, 3000)?;
    println!("Connected to server!");

    stream.write_all(b"Hello from client!")?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf)?;
    println!("Server responded: {}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
