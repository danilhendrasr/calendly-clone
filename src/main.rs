use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/hey",
            web::get().to(|| async { HttpResponse::Ok().body("Hey!") }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
