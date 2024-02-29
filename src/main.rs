use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

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
    let response_text = "Hello from the custom http server";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", response_text);
    stream.write_all(response.as_bytes())?;
    Ok(())
}
