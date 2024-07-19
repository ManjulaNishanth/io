use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Error, Write},
    path::{Path, PathBuf},
};

use serde_json::Value;

pub fn create_file(file_name: &PathBuf) -> Result<(), Error> {
    let create_file = File::create(file_name)?;
    println!("\n {create_file:?}");
    Ok(())
}

pub fn file_open(file_name: &PathBuf) -> Result<Option<String>, Error> {
    let open_file = fs::read(file_name)?;
    let file_to_string = String::from_utf8(open_file).unwrap();
    println!("\n {file_to_string:?}");
    Ok(Some(file_to_string))
}

pub fn write_metadata(
    file_name: &PathBuf,
    input: &HashMap<String, String>,
) -> Result<Option<HashMap<String, String>>, Error> {
    let input_to_string = serde_json::to_string(&input)?;
    fs::write(file_name, input_to_string).unwrap();
    let read_file = fs::read(file_name)?;
    let bytes_to_string = String::from_utf8(read_file).expect("Our bytes should be valid utf8");
    let content: HashMap<String, String> = serde_json::from_str(&bytes_to_string).unwrap();
    println!("\n {content:?}");
    Ok(Some(content))
}

pub fn apend_metadata(path: &PathBuf, input: &Value) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.clone())?;

    file.write(String::as_bytes(&serde_json::to_string(&input).unwrap()))?;

    let read = fs::read_to_string(path)?;
    println!("read : {read:?}");

    Ok(())
}

pub fn copy_file(origin: &Path, destination: &Path) -> Result<(), io::Error> {
    fs::copy(origin, destination)?;
    Ok(())
}

pub fn remove_file(origin: &Path) -> Result<(), io::Error> {
    fs::remove_file(origin)?;
    Ok(())
}

mod tests {
    use serde_json::json;
    use super::*;
    use std::path::Path;

    #[test]
    pub fn create_file_cfg() {
        let path = Path::new("./").join("data.json");
        println!("\n Create File");
        let created = create_file(&path).unwrap();
        assert_eq!(created, ())
    }

    #[test]
    pub fn file_open_cfg() {
        let path = Path::new("./").join("data.json");
        println!("\n File Initial Read");
        let created = file_open(&path).unwrap();
        assert_eq!(created, Some("".to_string()))
    }

    #[test]
    pub fn write_metadata_cfg() {
        let mut input = HashMap::new();
        input.insert("file_name".to_string(), "test.txt".to_string());

        let path = Path::new("./").join("data.json");
        println!("\n File Write Hashmap");
        let created = write_metadata(&path, &input).unwrap();
        assert_eq!(created, Some(input))
    }

    #[test]
    pub fn apend_metadata_cfg() {
        let input = json!({"file_name": "data2.txt"});
        let path = Path::new("./").join("data.json");
        println!("\n File Write Hashmap");
        let created = apend_metadata(&path, &input).unwrap();
        assert_eq!(created, ())
    }

    #[test]
    pub fn copy_file_cfg() {
        let from = Path::new("data.json");
        let to = Path::new("data_copy.txt");

        let result = copy_file(from, to).unwrap();
        assert_eq!(result, ())
    }

    #[test]
    pub fn copy_file_1_cfg() {
        let from = Path::new("data.json");
        let to = Path::new("data_copy1.txt");

        let result = copy_file(from, to).unwrap();
        assert_eq!(result, ())
    }

    #[test]
    pub fn remove_file_cfg() {
        let file_path = Path::new("data_copy.txt");

        let result = remove_file(file_path).unwrap();
        assert_eq!(result, ())
    }
}
