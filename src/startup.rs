use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    // .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}
