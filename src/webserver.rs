use actix_web::{ web, Responder, get, App };

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub async fn webserver() {
    actix_web::HttpServer::new(|| {
        App::new()
            .service(greet)
    }).bind("8080")
        .unwrap()
        .run()
        .await
        .unwrap();
}