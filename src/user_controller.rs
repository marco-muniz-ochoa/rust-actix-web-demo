use actix_web::{ web, get, post, HttpResponse, Responder };

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
       .service(save);
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("User list")
}

#[post("/")]
pub async fn save(request: String) -> impl Responder {
    HttpResponse::Ok().body(request)
}
