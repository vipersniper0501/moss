use std::fs;
use gethostname::gethostname;
use reqwest::{self};

pub mod linux;
use linux::*;
use moss_lib::*;

/// Gets config from a local file then runs the perform_checks
/// function to begin
fn local_mode(path: &str) {
    let path = fs::File::open(path).expect("Config file not found.");
    let config_data: MossData = serde_json::from_reader(path).unwrap();

    loop {
        let result_data: MossResults = perform_checks(&config_data);
        println!("{:#?}", result_data);
        println!("{:#?}", serde_json::to_string(&result_data));
        todo!();
    }
}

/// Gets config from a remote server then runs the perform_checks
/// function to begin
fn remote_mode(address: &str, teamid: String) {

    
    let binding = gethostname();
    let host = match binding.to_str() {
        Some(v) => v,
        None => {
            eprintln!("Failed to get hostname.");
            std::process::exit(1);
        }
    };
    println!("Hostname: {}", host);
    println!("\nAddress: {}", address);

    // Build url
    let mut url: String = "http://".to_owned();
    url.push_str(address);
    url.push_str("/api/v1/config/");
    url.push_str(host);

    let config_data: MossData = match reqwest::blocking::get(url) {
        Ok(v) => {
            match v.json() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to parse data: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to reach server: {}\nerror: {}", address, e);
            std::process::exit(1);
        }
    };

    println!("Pulled config_data: {:#?}", config_data);
    loop {
        let result_data: MossResults = perform_checks(&config_data);

        // Build the url
        let mut url = "http://".to_owned();
        url.push_str(address);
        url.push_str("/api/v1/results/");
        url.push_str(teamid.as_str());
        url.push('/');
        url.push_str(host);

        // Send the results to the server
        let client = reqwest::blocking::Client::new();
        let resp = client.post(url).json(&result_data).send().expect("Failed to send results.");

        println!("Response code: {}", resp.status());

        // Delay
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn print_usage() {
    println!("Usage:\tmoss-client [-L | --local] <path to local config>\n\t
        Or\n\tmoss-client [-R | --remote] <IP/Url of remote server> <teamid>");
}

/// Remote Mode attempts to form connection with server.
/// Connection Fails:
///     Exit and report failure
/// Connection Succeeds: 
///     Deserialize data
///     Data should contain the instructions for the checks
/// Main loop:
///     - Perform Checks
///     - Post checks to server
///     - repeat
fn main() {
    if std::env::args().len() < 3 {
        print_usage();
        std::process::exit(1);
    } 

    // Determine mode program should run in
    let mode = match std::env::args().nth(1) {
        Some(x) => x,
        None => {
            println!("Missing arguments.");
            // Should never reach this but who knows...
            print_usage();
            std::process::exit(1);
        },
    };

    // Arguments for the mode
    // Might want to move this into the match mode part to ensure correct
    // types of arguments. (i.e a path is actual a path, an URL/IP is actually
    // valid)
    let mode_argument = match std::env::args().nth(2) {
        Some(x) => x,
        None => {
            println!("Missing mode argument.");
            print_usage();
            std::process::exit(1);
        }
    };

    match mode.as_str() {
        "-L" | "--local" => {
            println!("Running moss-client in Local Mode.");
            local_mode(mode_argument.as_str());
        },
        "-R" | "--remote" => {
            println!("\nRunning moss-client in Remote Mode.");
            let teamid = match std::env::args().nth(3) {
                Some(v) => v,
                None => {
                    println!("Missing team id.");
                    print_usage();
                    std::process::exit(1);
                }
            };
            remote_mode(mode_argument.as_str(), teamid);
        },
        _ => {
            println!("Invalid arguments");
            print_usage();
            std::process::exit(1);
        }
    }

}
