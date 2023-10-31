use std::{fs::File, io::BufReader};
use actix_cors::Cors;
use actix_web::{web, App,HttpServer, http::header};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub mod handler;

use handler::*;

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file: File = match File::open("certificates/cert.pem") {
        Ok(v) => v,
        Err(e) => {
            std::eprintln!("\n\nFailed to find cert.pem in the certificates \
                            directory: certificates/cert.pem");
            std::eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let key_file: File = match File::open("certificates/key.pem") {
        Ok(v) => v,
        Err(e) => {
            std::eprintln!("\n\nFailed to find key.pem in the certificates \
                            directory: certificates/key.pem");
            std::eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // load TLS key/cert files
    let cert_reader = &mut BufReader::new(cert_file);
    let key_reader = &mut BufReader::new(key_file);

    // convert files to key/cert objects
    let cert_chain = match certs(cert_reader) {
        Ok(v) => v.into_iter().map(Certificate).collect(),
        Err(e) => {
            std::eprintln!("\n\nFailed to convert file into cert object.");
            std::eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let mut keys: Vec<PrivateKey> = match pkcs8_private_keys(key_reader) {
        Ok(v) => v.into_iter().map(PrivateKey).collect(),
        Err(e) => {
            std::eprintln!("\n\nFailed to convert file into cert object.");
            std::eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    match config.with_single_cert(cert_chain, keys.remove(0)) {
        Ok(v) => return v,
        Err(e) => {
            std::eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}


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



    // closure to create new app
    let app = |xpool| {
        App::new()
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
    };

    let args: Vec<String> = std::env::args().collect();
    let uflag: String = "--unsecure".to_string();
    let unsecure_flag: bool = args.contains(&uflag);

    if unsecure_flag {
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
            let app = app(xpool);
            return app.wrap(cors);
        })
        .bind(("0.0.0.0", 4224))?
        .run()
        .await
    }
    else {
        let config = load_rustls_config();

        println!("Listening on https://127.0.0.1:4224");
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
            let app = app(xpool);
            return app.wrap(cors);
        })
        .bind_rustls_021("0.0.0.0:4224", config)?
        .run()
        .await

    }
}
