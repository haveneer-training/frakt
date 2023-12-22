use std::{io::{self, Read}, net::TcpListener};

fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8067")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Incoming connection {stream:?}");

        let mut lenbuf = [0;4];
        stream.read_exact(&mut lenbuf)?;
        let len = u32::from_be_bytes(lenbuf);
        let mut sbuf = vec![0_u8; len as usize];
        stream.read(&mut sbuf)?;
        // println!("{sbuf:?}");
        let s = String::from_utf8_lossy(&sbuf);
        println!("{s}")
    }
        
    Ok(())
}
