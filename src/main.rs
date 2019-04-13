use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("couldn't bind to address");

    for stream in listener.incoming() {
        match stream.and_then(handle_client) {
            Ok(_) => {
                println!("client disconnected gracefully");
            }
            Err(e) => {
                println!("client error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("new client: {}", stream.peer_addr()?);
    writeln!(stream, "hello client")?;

    let mut buffer = [0; 10];
    stream.read_exact(&mut buffer)?;
    
    if buffer == [65; 10] {
        println!("yeh");
    } else {
        println!("oh");
    }

    Ok(())
}
