//! Linux functions

use std::{path::Path, fs::{self, Metadata}};

use crate::structures::*;

pub fn perform_checks(config_data: &MossData) -> MossResults {

    let mut results: MossResults = MossResults::new();

    // Files
    results = detect_approved_files(&config_data, results);
    results = detect_invalid_files(&config_data, results);

    // Valid Users

    // Invalid Users

    return results;
}

// fn detect_file(path: &str) -> Result<bool, std::io::Error> {
fn detect_file(path: &str) -> Result<bool, std::io::Error> {
    // let path = Path::new(path);

    let path = fs::metadata(path)?;
    // Note: This will fail if the current user does not have permission to read
    // the file!
    // if !path.is_file() {
        // return Err(std::io::Error::new(
                // std::io::ErrorKind::NotFound,
                // "File not found."
        // ));
    // }

    // Ok(true)

    return Ok(path.is_file());
}

fn detect_approved_files(config_data: &MossData, mut results: MossResults) -> MossResults {
    
    for files in config_data.approved_files.iter() {
        let path = files.location.clone();
        let status = match detect_file(path.as_str()) {
            Ok(x) => x,
            Err(..) => false
        };
        results.approved_files.push(MossFilesResults::new(
                files.name.clone(),
                status
                )
            );
    }

    return results;
}

fn detect_invalid_files(config_data: &MossData, mut results: MossResults) -> MossResults {
    for files in config_data.invalid_files.iter() {
        let path = files.location.clone();
        let status = match detect_file(path.as_str()) {
            Ok(x) => {
                if x {
                    false
                } else {
                    true
                }
            },
            Err(..) => true
        };
        results.invalid_files.push(MossFilesResults::new(
                files.name.clone(),
                status
                )
            );
    }

    return results;
}

fn _list_users() {

}

fn _detect_valid_users() {

}

fn _detect_invalid_users() {

}


// Tests Coming Soon!

