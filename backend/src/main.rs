use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn get_json() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, World!",
        "status": "success"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/json", web::get().to(get_json))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
