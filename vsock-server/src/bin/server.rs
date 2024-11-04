use std::io::{Read, Write};
use vsock_server::vsock::VsockListener;

fn main() -> std::io::Result<()> {
    let listener = VsockListener::bind(3000)?;
    println!("Server listening on port 3000...");

    let mut stream = listener.accept()?;
    println!("Client connected!");

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf)?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    stream.write_all(b"Hello from server!")?;
    Ok(())
}
