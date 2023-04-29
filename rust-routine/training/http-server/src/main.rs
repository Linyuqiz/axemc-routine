mod handler;
mod router;
mod server;

fn main() {
    server::Server::new("127.0.0.1:3000").run()
}
