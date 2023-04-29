use crate::router::Router;
use http::http_request::HttpRequest;
use std::io::Read;
use std::net::TcpListener;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(self) {
        let connection = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);

        for stream in connection.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();
            let request: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(request, &mut stream);
        }
    }
}
