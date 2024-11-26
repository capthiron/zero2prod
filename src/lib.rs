use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| 
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
        )
        .listen(listener)?
        .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscriptionFormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<SubscriptionFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
