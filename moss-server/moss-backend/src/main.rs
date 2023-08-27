use actix_web::{web, App,HttpServer};
use dotenv::dotenv;
use mysql::Pool;

pub mod handler;

use handler::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Notes: Perhaps, I should take in arguments for
    // the database password.
    //
    // Should also ensure database can be connected to before the 
    // backend can even turn on.
    
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Could not find mysql database url in .env file");

    if database_url.contains("root") {
        println!("Warning: It would be a smart idea to change the user from 
            root to some other user with your database.");
    }
    if database_url.contains("password") {
        println!("Warning: It would be wise to change the default password 
            for the database from password!");
    }

    // MySQL database setup
    let pool: Pool = Pool::new(database_url.as_str())
        .expect("Failed to create a connection pool");


    println!("Listening on http://127.0.0.1:4224");
    HttpServer::new(move || {
        let pool = pool.clone();
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: pool
            }))
            .service(test_response)
            .service(submit_results)
            .service(get_config)
            .service(update_config)
            .service(create_teams)
    })
    .bind(("0.0.0.0", 4224))?
    .run()
    .await
}
