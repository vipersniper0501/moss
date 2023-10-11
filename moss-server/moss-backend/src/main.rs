use actix_cors::Cors;
use actix_web::{web, App,HttpServer, http::header};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;


pub mod handler;

use handler::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Notes: Perhaps, I should take in arguments for
    // the database password.
    
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Could not find mysql database url in .env file {DATABASE_URL=\"my_database_url_here\"}");

    if database_url.contains("root") {
        println!("Warning: It would be a smart idea to change the user from \
            root to some other user with your database.");
    }
    if database_url.contains("password") {
        println!("Warning: It would be wise to change the default password \
            for the database from password!");
    }

    // MySQL database setup
    let xpool = match MySqlPool::connect(&database_url).await {
            Ok(v) => {
                v
            }
            Err(e) => {
                eprintln!("Failed to create a connection pool (sqlx): {}", e);
                std::process::exit(1);
            }
        };

    println!("Listening on http://127.0.0.1:4224");

    HttpServer::new(move || {
        let xpool = xpool.clone();
        let cors = Cors::default()
            // Allows connection from local only frontend
            // Need to figure out way to accept from other server locations...
            .allowed_origin("http://127.0.0.1:4223")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                db_xpool: xpool
            }))
            .service(test_response)
            .service(submit_results)
            .service(get_results)
            .service(get_config)
            .service(update_config)
            .service(create_teams)
            .service(remove_team)
            .service(remove_multiple_teams)
            .service(get_teams)
            .service(get_systems)
    })
    .bind(("0.0.0.0", 4224))?
    .run()
    .await
}
