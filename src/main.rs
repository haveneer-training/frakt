use std::io;
use std::net::TcpStream;
use std::io::Write;


fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8067")?;
    let n: u32 = 258;

    let s = "Hello from the client";
    let n = s.len() as u32;
    stream.write(&n.to_be_bytes())?;
    stream.write(s.as_bytes())?;

    Ok(())
}
