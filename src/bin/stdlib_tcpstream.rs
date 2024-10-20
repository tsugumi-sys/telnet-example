use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2323").unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();

        let mut buf = [0; 128];
        let n = stream.read(&mut buf).unwrap();
        std::io::stdout().write_all(&buf[..n]).unwrap();
    }
}
