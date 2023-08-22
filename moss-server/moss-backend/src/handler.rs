use actix_web::{get, post, web, Responder, HttpResponse};

use moss_lib::MossResults;
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;


pub struct AppState {
    pub db_pool: Pool,
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


/// 
///
/// * `app_data`: 
fn get_number_of_teams(app_data: &web::Data<AppState>) -> Option<Result<i32, Box<dyn std::error::Error>>> {
    let pool = app_data.db_pool.clone();

    match pool.get_conn() {
        Ok(mut v) => {

            match v.query_first::<i32, &str>(
                "SELECT MAX(TeamID) \
                 FROM Teams"
            ) {
                Ok(result) => {
                    match result {
                        Some(value) => {
                            return Some(Ok(value));
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Err(e) => {
                    return Some(Err(Box::new(e)));
                }
            }
        }
        Err(e) => {
            return Some(Err(Box::new(e)));
        }
    }

}

/// Gets the list of operating systems that are being monitored from the
/// database.
///
/// Pre-req: Requres at least one team with id value of 1
///
/// * `app_data`: The AppState of the program that contains global data
fn get_db_ops(app_data: &web::Data<AppState>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    
    // let app_data = app_data.clone();
    // let app_data = app_data.into_inner();
    let pool = app_data.db_pool.clone();

    match pool.get_conn() {
        Ok(mut v) => {
            match v.query_map (
                "SELECT OperatingSystem \
                 FROM Configurations \
                 WHERE TeamID = 1",
                 |operating_system: String| operating_system,
            ) {
                Ok(v) => {
                    return Ok(v);

                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}

#[post("/api/v1/submit_results/{team_id}/{system}")]
pub async fn submit_results(path_data: web::Path<(i32, String)>,
    app_data: web::Data<AppState>, results: web::Json<MossResults>) -> impl Responder {

    let (team_id, system) = path_data.into_inner();

    let ops;
    match get_db_ops(&app_data) {
        Ok(v) => {ops = v}
        Err(e) => {
            return HttpResponse::ExpectationFailed()
                .body(
                    format!("Failed to get operating systems from the database: {}", e)
                    )
        }
    }

    if !ops.contains(&system) {
        return HttpResponse::BadRequest()
            .body(format!("System {} is not in database.", system));
    }

    let teams_amount: i32;
    match get_number_of_teams(&app_data) {
        Some(v) => {
            match v {
                Ok(result) => {
                    teams_amount = result;
                }
                Err(e) => {
                    return HttpResponse::ExpectationFailed()
                        .body(format!("Error contacting database: {}", e));
                }
            }
        }
        None => {
            return HttpResponse::ExpectationFailed()
                .body("Failed because there are no teams in the database.");
        }
    }

    if team_id > teams_amount || team_id < 1 {
        return HttpResponse::BadRequest()
            .body(format!("Team {} does not exist in the database.", team_id));
    }

    let results = results.into_inner();
    let results_json;
    match serde_json::to_string(&results) {
        Ok(v) => results_json = v,
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(format!("Failed to serialize struct into json: {}", e));
        }
    }

    let pool = app_data.db_pool.clone();
    match pool.get_conn() {
        Ok(mut v) => {
            match v.exec_drop(
                "UPDATE Results \
                 SET ResultData = :result_json \
                 WHERE TeamID = :team_id AND OperatingSystem = :operating_system",
                params!{
                    "result_json" => results_json,
                    "team_id" => team_id,
                    "operating_system" => system
                }
            ) {
                Ok(()) => {/*Do nothing if success*/},
                Err(e) => {
                    return HttpResponse::BadRequest()
                        .body(format!("Failed to insert into table: {}", e));
                }
            }
        }
        Err(e) => {
            return HttpResponse::ExpectationFailed()
                .body(format!("Failed to get connection from pool: {}", e));

        }
    }

    HttpResponse::Ok().body("Success")
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
                .body(format!("Failed to get connection from pool: {}", e));
        }
    }

    HttpResponse::Ok().body("Success")
}


#[get("/api/v1/test_response")]
pub async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}
