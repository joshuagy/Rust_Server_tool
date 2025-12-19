use std::io::{Read, Write};
use native_tls::TlsConnector;
use std::net::TcpStream;

fn secure_connection() -> Result<bool, Box<dyn std::error::Error>> {
    let connector = TlsConnector::new()?;
    let stream = TcpStream::connect("www.rust-lang.org:443")?;
    let mut stream = connector.connect("www.rust-lang.org", stream)?;

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n")?;
    let mut res = vec![];
    stream.read_to_end(&mut res)?;
    println!("Response: {}", stringify!(&res));
    Ok(true)
}

fn main() {
    println!("Hello from client !");
    secure_connection().unwrap();
}