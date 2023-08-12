use actix_web::{get, post, web, Responder, HttpResponse};

use moss_lib::MossResults;

// Future idea for how to handle request from clients: // /api/v1/{team_id}/{system}/{method}
// An example would be: /api/v1/1/ubuntu_18/get_config
// Here, team 1 is requesting the config for the ubuntu_18 system.

#[post("/api/v1/test_handler")]
pub async fn test_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an external handler from the handler.rs file!")
}

#[post("/api/v1/submit_results")]
pub async fn submit_results(results: web::Json<MossResults>) -> impl Responder {
    println!("Results:\n{results:#?}");
    HttpResponse::Ok().body("Recieved results.")
}


#[get("/api/v1/test_response")]
pub async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}