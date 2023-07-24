//! Linux functions

use std::fs;

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

    let path = fs::metadata(path)?;

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

    // temp
    // list_users();
    return results;
}

fn list_users() -> Vec<LinuxUserData> {
    let mut users: Vec<LinuxUserData> = Vec::new();

    // Read the output of the cat of the file
    let output = std::process::Command::new("cat")
        .arg("/etc/passwd")
        .output()
        .expect("Failed to execute cat command on /etc/passwd");

    let output = String::from_utf8_lossy(&output.stdout);
    let output: Vec<&str> = output.split('\n').collect();

    let output_iter = output.iter();

    // Build the user
    for s in output_iter {
        let data: Vec<&str> = s.split(':').collect();

        // Fixes bug with the output.split that leaves a trailing entry. 
        // Print out output if you don't believe me.
        if data.len() < 7 {
            break;
        }

        let mut new_user: LinuxUserData = LinuxUserData::new();
        new_user.name = data[0].to_string();
        new_user.has_password = match data[1] {
            "x" => true,
            _ => false,
        };
        new_user.uid = data[2].parse::<i32>()
            .expect("Failed to convert uid to i32");
        new_user.gid = data[3].parse::<i32>()
            .expect("Failed to convert gid to i32");
        new_user.home_dir = data[5].to_string();
        new_user.shell = data[6].to_string();

        users.push(new_user);
    }

    return users;
}

fn _detect_valid_users() {

}

fn _detect_invalid_users() {

}


// Tests Coming Soon!

#[cfg(test)]
mod tests {
    use crate::{structures::*,
        linux::list_users};

    #[test]
    fn test_list_users() {
        let data: Vec<LinuxUserData> = list_users();
        assert_eq!(data[0].name, "root");
        assert_eq!(data[0].has_password, true);
        assert_eq!(data[0].uid, 0);
    }
    
}
