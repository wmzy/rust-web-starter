use actix_web::{App, HttpServer};
use listenfd::ListenFd;

mod core;
mod app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(app::route()));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:4000")?
    };

    server.run().await
}
