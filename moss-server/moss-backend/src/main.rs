use std::{fs::File, io::BufReader};
use actix_cors::Cors;
use actix_web::{web, App,HttpServer, http::header};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

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

    let config = load_rustls_config();

    println!("Listening on http://127.0.0.1:4224");

    HttpServer::new(move || {
        let xpool = xpool.clone();
        let cors = Cors::default()
            // Allows connection from local only frontend
            // Need to figure out way to accept from other server locations...
            .allow_any_origin()
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
    // .bind(("0.0.0.0", 4224))?
    .bind_rustls_021("0.0.0.0:4224", config)?
    .run()
    .await
}


fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certificates/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("certificates/key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
