use actix_web::{get, post, web, Responder, HttpResponse};

use moss_lib::MossResults;
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;


pub struct AppState {
    pub db_pool: Pool
}

#[derive(Deserialize)]
pub struct OpsList {
    operating_systems: Vec<String>
}


#[get("/api/v1/get_config/{team_id}/{system}")]
pub async fn get_team_config(path_data: web::Path<(i32, String)>) -> impl Responder {
    let (team_id, system) = path_data.into_inner();

    HttpResponse::Ok().body(format!("<Config data for team {team_id}'s {system} system goes here>"))
}

#[post("/api/v1/submit_results/{team_id}/{system}")]
pub async fn submit_results(path_data: web::Path<(i32, String)>,results: web::Json<MossResults>) -> impl Responder {
    let (team_id, system) = path_data.into_inner();
    let results = results.into_inner();


    println!("Results:\n{results:#?}");
    HttpResponse::Ok().body("Recieved results.")
}

// Note: This should only be called once when first setting up admin dashboard.
#[post("/api/v1/create_teams/{amount}")]
pub async fn create_teams(path_data: web::Path<i32>, app_data: web::Data<AppState>,
    ops_list: web::Json<OpsList>) -> impl Responder {

    let amount = path_data.into_inner();
    let ops: Vec<String> = ops_list.into_inner().operating_systems;

    let app_data = app_data.into_inner();
    let pool = app_data.db_pool.clone();
    match pool.get_conn() {
        Ok(mut v) => {
            for i in 1..=amount {
                // let params = params![format!("Team {}", i)];
                // v.exec_drop(r"INSERT INTO Teams (TeamName) VALUES (?)", (format!("Team {}", i),))
                    // .expect("Failed to insert into table");
                match v.exec_drop(
                    "INSERT INTO Teams (TeamID, TeamName)\
                     VALUES (:team_id, :team_name)",
                    params!{
                        "team_id" => i,
                        "team_name" => format!("Team {}", i),
                    }
                ) {
                    Ok(()) => {/*Do nothing if success*/},
                    Err(e) => {
                        return HttpResponse::BadRequest()
                            .body(format!("Failed to insert into table: {}", e));
                    }
                }

                for x in 0..ops.len() {
                    match v.exec_drop(
                        "INSERT INTO Configurations (TeamID, OperatingSystem)\
                         VALUES (:team_id, :operating_system)",
                         params! {
                             "team_id" => i,
                             "operating_system" => ops[x as usize].clone(),
                         }
                    ) {
                        Ok(()) => {/*Do nothing if success*/},
                        Err(e) => {
                            return HttpResponse::BadRequest()
                                .body(format!("Failed to insert into configuration table: {}", e));
                        }

                    }

                    match v.exec_drop(
                        "INSERT INTO Results (TeamID, OperatingSystem)\
                         VALUES (:team_id, :operating_system)",
                         params! {
                             "team_id" => i,
                             "operating_system" => ops[x as usize].clone(),
                         }
                    ) {
                        Ok(()) => {/*Do nothing if success*/},
                        Err(e) => {
                            return HttpResponse::BadRequest()
                                .body(format!("Failed to insert into results table: {}", e));
                        }
                    }
                }
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


#[get("/api/v1/test_response")]
pub async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}
