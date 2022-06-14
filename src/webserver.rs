use actix_web::{ web, Responder, get, App, HttpResponse, http::header::ContentType };

use crate::CACHE;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/proxies")]
async fn proxies() -> impl Responder {
    let lock = CACHE.lock().unwrap();
    let list = serde_json::to_string(&lock.inner.as_ref()).unwrap();
    // Send the list of proxies to the client as json
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(list)
}

// #[actix_web::main]
pub async fn webserver() {
    actix_web::HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(proxies)
    }).bind(("127.0.0.1", 80))
        .unwrap()
        .run()
        .await
        .unwrap();
}