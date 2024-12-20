use server::Server;

mod server;
mod handler;
mod router;

fn main() {
    let server = Server::new("127.0.0.1:9660");
    server.run();
}
