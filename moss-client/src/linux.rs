//! Linux functions

use std::fs;
use std::path::Path;

fn perform_checks() {

}

fn detect_file(path: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(path);

    // Note: This will fail if the current user does not have permission to read
    // the file!
    if !path.is_file() {
        return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found."
        ));
    }


    Ok(true)
}

fn list_users() {

}

fn detect_valid_users() {

}

fn detect_invalid_users() {

}
