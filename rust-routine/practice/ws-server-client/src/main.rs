use std::{net::TcpListener, thread::spawn};

use tungstenite::{accept, connect, Message};
use url::Url;

fn main() {
    spawn(|| ws_server());

    let (mut socket, response) =
        connect(Url::parse("ws://127.0.0.1:9001").unwrap()).expect("Can't connect");

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .write_message(Message::Text("Hello, Test!".into()))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {:?}", msg);
    }
}

fn ws_server() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                println!("{msg}");

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
