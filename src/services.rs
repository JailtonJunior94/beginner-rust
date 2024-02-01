use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;

#[get("/health")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "Health is running";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health);
    conf.service(scope);
}
