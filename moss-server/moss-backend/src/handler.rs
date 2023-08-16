use actix_web::{get, post, web, Responder, HttpResponse};

use moss_lib::MossResults;
use mysql::*;
use mysql::prelude::*;


pub struct AppState {
    pub db_pool: Pool
}


// Future idea for how to handle request from clients: // /api/v1/{team_id}/{system}/{method}
// An example would be: /api/v1/1/ubuntu_18/get_config
// Here, team 1 is requesting the config for the ubuntu_18 system.

#[get("/api/v1/get_config/{team_id}/{system}")]
pub async fn get_team_config(path_data: web::Path<(i32, String)>) -> impl Responder {
    let (team_id, system) = path_data.into_inner();

    HttpResponse::Ok().body(format!("<Config data for team {team_id}'s {system} system goes here>"))
}

#[post("/api/v1/submit_results/{team_id}/{system}")]
pub async fn submit_results(path_data: web::Path<(i32, String)>,results: web::Json<MossResults>) -> impl Responder {
    let (team_id, system) = path_data.into_inner();


    println!("Results:\n{results:#?}");
    HttpResponse::Ok().body("Recieved results.")
}

#[post("/api/v1/create_teams/{amount}")]
pub async fn create_teams(path_data: web::Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let amount = path_data.into_inner();
    let app_data = app_data.into_inner();
    let pool = app_data.db_pool.clone();
    match pool.get_conn() {
        Ok(mut v) => {
            for i in 1..=amount {
                // let params = params![format!("Team {}", i)];
                v.exec_drop(r"INSERT INTO Teams (TeamName) VALUES (?)", (format!("Team {}", i),))
                    .expect("Failed to insert into table");
            }
        },
        Err(e) => {
            return HttpResponse::ExpectationFailed()
                .body(format!("Failed to get connection from pool: {}",
                        e.to_string()));
        }
    }

    HttpResponse::Ok().body("Success")
}


#[post("/api/v1/test_handler")]
pub async fn test_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an external handler from the handler.rs file!")
}


#[get("/api/v1/test_response")]
pub async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}
