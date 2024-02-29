use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

use serde::Serialize;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("Server is running on http://localhost:4000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "Accepted a new connection: {}",
                    stream.local_addr().unwrap()
                );
                handle_connection(stream)?;
            }
            Err(err) => println!("couldn't get client: {err:?}"),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    let response_json = serde_json::to_string(&user)?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        response_json
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct User {
    name: String,
    age: u8,
}
