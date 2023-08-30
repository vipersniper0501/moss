use actix_web::{get, post, web, Responder, HttpResponse};
use moss_lib::{MossResults, MossData};
use serde::Deserialize;


pub struct AppState {
    pub db_xpool: sqlx::Pool<sqlx::MySql>
}

#[derive(Deserialize)]
pub struct OpsList {
    operating_systems: Vec<String>
}

/// Takes a system name and compares it to the database to see if it is already
/// in the database.
///
/// * `system`: Name of operating system
/// * `app_data`: Structure containing app state
async fn validate_system(system: &String, app_data: &web::Data<AppState>) -> Result<(), HttpResponse> {
    let ops = match get_db_ops(app_data).await {
        Ok(v) => v,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body(
                    format!("Failed to get operating systems from the database: {}", e)
                    ));
        }
    };
    if !ops.contains(system) {
        return Err(HttpResponse::BadRequest()
            .body(format!("System {} is not in database.", system)));
    }
    Ok(())
}

/// Takes a team id and compares it to the database to see if it is already in
/// the database.
///
/// * `team_id`: 
/// * `app_data`: Structure containing app state
async fn validate_team(team_id: i32, app_data: &web::Data<AppState>) -> Result<(), HttpResponse> {
    let teams_amount: i32 = match get_number_of_teams(&app_data).await {
            Ok(result) => result,
            Err(e) => {
                return Err(HttpResponse::InternalServerError()
                    .body(format!("Error contacting database: {}", e)));
            }
    };

    if team_id > teams_amount || team_id < 1 {
        return Err(HttpResponse::BadRequest()
            .body(format!("Team {} does not exist in the database.", team_id)));
    }
    Ok(())
}

#[post("/api/v1/update_config/{system}")]
pub async fn update_config(path_data: web::Path<String>, app_data: web::Data<AppState>,
config: web::Json<MossData>) -> impl Responder {
    let system = path_data.into_inner();

    if let Err(response) = validate_system(&system, &app_data).await {
        return response;
    }

    let config = config.into_inner();
    let config_json = match serde_json::to_string(&config) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(format!("Failed to serialize struct into json: {}", e));
        }
    };

    let pool = app_data.db_xpool.clone();

    match sqlx::query!(
        "UPDATE Configurations \
         SET configuration_data = ? \
         WHERE operating_system = ?",
         config_json,
         system
    ).execute(&pool).await {
        Ok(_v) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to execute query on database: {}", e));
        }
    }

    HttpResponse::Ok().body("Sucess")
}


#[get("/api/v1/get_config/{system}")]
pub async fn get_config(path_data: web::Path<String>, app_data: web::Data<AppState>) -> impl Responder {
    let system = path_data.into_inner();

    if let Err(response) = validate_system(&system, &app_data).await {
        return response;
    }

    let pool = app_data.db_xpool.clone();

    let result = match sqlx::query!(
        "SELECT configuration_data \
         FROM Configurations \
         WHERE operating_system = ?",
         system
    ).fetch_one(&pool).await {
        Ok(v) => {
            match v.configuration_data {
                Some(result) => result.to_string(),
                None => "No data".to_string()
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to execute query on database: {}", e));
        }
    };

    HttpResponse::Ok().body(result)
}


/// Gets the number of teams that are in the database.
///
/// * `app_data`: The AppState of the program that contains global data
async fn get_number_of_teams(app_data: &web::Data<AppState>) -> Result<i32, Box<dyn std::error::Error>> {
    let pool = app_data.db_xpool.clone();

    let result: i32 = match sqlx::query!(
        "SELECT MAX(team_id) AS max_team_id \
         FROM Teams"
    ).fetch_one(&pool).await {
        Ok(v) => {
            match v.max_team_id {
                Some(result) => result,
                None => 0
            }
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    return Ok(result);
}

/// Gets the list of operating systems that are being monitored from the
/// database.
///
/// Pre-req: Requres at least one team with id value of 1
///
/// * `app_data`: The AppState of the program that contains global data
async fn get_db_ops(app_data: &web::Data<AppState>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    
    let pool = app_data.db_xpool.clone();

    let result: Vec<String> = match sqlx::query!(
        "SELECT operating_system \
         FROM Configurations"
    ).fetch_all(&pool).await {
        Ok(v) => {
            let operating_systems: Vec<String> = v.iter()
                .map(|row|
                    match &row.operating_system {
                        Some(x) => x.to_string(),
                        None => "No data".to_string()
                    }
                    ).collect();
            operating_systems
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    Ok(result)
}


#[post("/api/v1/submit_results/{team_id}/{system}")]
pub async fn submit_results(path_data: web::Path<(i32, String)>,
    app_data: web::Data<AppState>, results: web::Json<MossResults>) -> impl Responder {

    let (team_id, system) = path_data.into_inner();

    if let Err(response) = validate_team(team_id, &app_data).await {
        return response;
    }

    if let Err(response) = validate_system(&system, &app_data).await {
        return response;
    }

    let results = results.into_inner();
    let results_json = match serde_json::to_string(&results) {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::BadRequest()
                .body(format!("Failed to serialize struct into json: {}", e));
        }
    };

    let pool = app_data.db_xpool.clone();
    match sqlx::query!(
        "UPDATE Results \
         SET result_data = ? \
         WHERE team_id = ? AND operating_system = ?",
         results_json,
         team_id,
         system
    ).execute(&pool).await {
        Ok(_v) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to execute query on database: {}", e));
        }
    }

    HttpResponse::Ok().body("Success")
}


#[get("/api/v1/get_results/{team_id}/{system}")]
pub async fn get_results(path_data: web::Path<(i32, String)>, app_data: web::Data<AppState>)
    -> impl Responder {

    let (team_id, system) = path_data.into_inner();

    if let Err(response) = validate_team(team_id, &app_data).await {
        return response;
    }

    if let Err(response) = validate_system(&system, &app_data).await {
        return response;
    }

    let pool = app_data.db_xpool.clone();
    
    let result = match sqlx::query!(
        "SELECT result_data \
         FROM Results \
         WHERE team_id = ? AND operating_system = ?",
         team_id,
         system
    ).fetch_one(&pool).await {
        Ok(v) => {
            match v.result_data {
                Some(result) => result.to_string(),
                None => "No data".to_string()
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to execute query on database: {}", e));
        }
    };

    HttpResponse::Ok().body(result)
}


// Note: This should only be called once when first setting up admin dashboard.
#[post("/api/v1/create_teams/{amount}")]
pub async fn create_teams(path_data: web::Path<i32>, app_data: web::Data<AppState>,
    ops_list: web::Json<OpsList>) -> impl Responder {

    let amount = path_data.into_inner();
    let teams_amount = match get_number_of_teams(&app_data).await{
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get number of teams from database: {}", e));
        }
    };
    let ops: Vec<String> = ops_list.into_inner().operating_systems;

    let pool = app_data.db_xpool.clone();

    for i in (1 + teams_amount)..=(amount + teams_amount) {
        match sqlx::query!(
            "INSERT INTO Teams (team_id, team_name) \
             VALUES (?, ?)",
             i,
             format!("Team {}", i)
        ).execute(&pool).await {
            Ok(_v) => {}
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to execute query on database: {}", e));
            }
        };

        for x in 0..ops.len() {
            // Change this

            match sqlx::query!(
                "INSERT INTO Results (team_id, operating_system) \
                 VALUES (?, ?)",
                 i,
                 ops[x as usize].clone()
            ).execute(&pool).await {
                Ok(_v) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to execute query on database: {}", e));
                }
            };
        }
    }
    for x in 0..ops.len() {
        match sqlx::query!(
            "INSERT INTO Configurations (operating_system) \
             VALUES (?)",
             ops[x as usize].clone()
        ).execute(&pool).await {
            Ok(_v) => {}
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to execute query on database: {}", e));
            }
        };

    }

    HttpResponse::Ok().body("Success")
}


#[get("/api/v1/test_response")]
pub async fn test_response() -> impl Responder {
    HttpResponse::Ok().body("Test response from Moss server backend!")
}
