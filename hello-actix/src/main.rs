use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Tok!")
}

async fn index_two() -> impl Responder {
    HttpResponse::Ok().body("Hello Kamata again!")
}

#[get("/hello")]
async fn index_three() -> impl Responder {
    HttpResponse::Ok().body("Hey thereeee!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index_two))
            .service(index_three)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:4000")?
    };

    server.run().await
}
