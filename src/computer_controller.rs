use actix_web::{ web, error, get, post, Error, HttpRequest, HttpResponse, Responder };
use serde::{ Serialize, Deserialize };
use futures::future::{ ready, Ready };

#[derive(Serialize, Deserialize, Debug)]
struct Computer {
    id: i64,
    model: String,
    brand: String,
}

impl Responder for Computer {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _request: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub fn route_config(cfg: &mut web::ServiceConfig) {
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err,
                HttpResponse::Conflict().finish()).into()
        });

    cfg.app_data(json_config)
        .service(index)
        .service(save);
}

#[get("/")]
async fn index() -> impl Responder {
    Computer { 
        id: 1001, 
        model: "MacBook Pro 15-inch 2018".to_string(),
        brand: "Apple".to_string(),
    }
}

#[post("/")]
async fn save(computer: web::Json<Computer>) -> impl Responder {
    println!("{:?}", &computer);

    computer
}
