// write http server
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Hello, World!</h1></body></html>\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
use tungstenite::{accept, Message};
use url::Url;

fn run_websocket_server() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("WebSocket server listening on ws://127.0.0.1:9001");

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
fn run_websocket_client() {
    let (mut socket, _) = tungstenite::connect(Url::parse("ws://127.0.0.1:9001").unwrap()).expect("Can't connect");
    socket.write_message(Message::Text("Hello, WebSocket!".into())).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}

// Uncomment one of these in the main function to run either the server or client
// run_websocket_server();
// run_websocket_client();

fn main() {
    // run_server();
    run_websocket_server();

    run_websocket_client();
}

