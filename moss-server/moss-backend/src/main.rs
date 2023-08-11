use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use moss_lib::MossResults;


#[post("/api/v1/submit_results")]
async fn submit_results(results: web::Json<MossResults>) -> impl Responder {
    println!("Results:\n{results:#?}");
    HttpResponse::Ok().body("Recieved results.")
}


#[get("/api/v1/test_response")]
async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:4224");
    HttpServer::new(|| {
        App::new()
            .service(test_response)
            .service(submit_results)
    })
    .bind(("127.0.0.1", 4224))?
    .run()
    .await
}
