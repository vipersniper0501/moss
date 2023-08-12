use actix_web::{App, HttpResponse, HttpServer, Responder};

pub mod handler;
use handler::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:4224");
    HttpServer::new(|| {
        App::new()
            .service(test_response)
            .service(submit_results)
            .service(test_handler)
            .service(get_team_config)
    })
    .bind(("0.0.0.0", 4224))?
    .run()
    .await
}
