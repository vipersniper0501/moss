use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct MossData {
    pub server: String,
    pub approved_files: Vec<MossFileData>,
    pub invalid_files: Vec<MossFileData>,
    pub valid_users: Vec<String>,
    pub invalid_users: Vec<String>
}

#[derive(Deserialize, Debug, Default)]
pub struct MossFileData {
    pub name: String, 
    pub location: String
}


#[derive(Serialize, Debug, Default)]
pub struct MossResults {
    pub approved_files: Vec<MossFilesResults>,
    pub invalid_files: Vec<MossFilesResults>,
    pub valid_users: Vec<bool>,
    pub invalid_users: Vec<bool>
}

impl MossResults {

    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Debug)]
pub struct MossFilesResults {
    pub name: String,
    pub status: bool
}

impl MossFilesResults {
    pub fn new(n: String, s: bool) -> Self {
        Self { name: n, status: s }
    }
}



// Need to figure out return structure for server...

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_deserialize_server() {
        let input = fs::File::open("tmp/test.json").expect("No file found");
        let deserialized_data: MossData = serde_json::from_reader(input).unwrap();
        assert_eq!(deserialized_data.server, "127.0.0.1");
        assert_eq!(deserialized_data.approved_files.len(), 3);
        assert_eq!(deserialized_data.approved_files[0].name, "neovim");
        assert_eq!(deserialized_data.approved_files[2].location, "/bin/nano");
        assert_eq!(deserialized_data.valid_users[0], "viper");
        assert_eq!(deserialized_data.invalid_users[0], "jake");
        
    }
}
