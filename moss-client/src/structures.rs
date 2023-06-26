use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct MossData {
    server: String,
    files: Vec<MossFileData>,
    valid_users: Vec<String>,
    invalid_users: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct MossFileData {
    name: String, 
    location: String
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
        assert_eq!(deserialized_data.files.len(), 2);
        assert_eq!(deserialized_data.files[0].name, "neovim");
        assert_eq!(deserialized_data.files[1].location, "/bin/nano");
        assert_eq!(deserialized_data.valid_users[0], "viper");
        assert_eq!(deserialized_data.invalid_users[0], "jake");
        
    }
}
