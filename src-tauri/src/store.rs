use std::fs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::chi_models::{ChiLibraryNode, Action, ChiConfig};

#[derive(Serialize, Deserialize, Debug)]
struct FileStore {
    paths: Vec<String>
}

pub fn get_config() -> ChiConfig {
    ChiConfig { 
        methods: get_methods(),
        libraries: get_libs()
    }
}

pub fn get_methods() -> Vec<String> {
    vec![String::from("GET"), String::from("POST"), String::from("PUT"), String::from("DELETE")]
}

pub fn get_libs() -> Vec<ChiLibraryNode> {
    let store = get_filestore();
    let result: Vec<ChiLibraryNode> = store.paths.iter()
        .map(|path| get_file_content(path))
        .map(|content| serde_json::from_str::<ChiLibraryNode>(&content).unwrap())
        .collect();
    result
}

pub fn create_action(path: String) -> Vec<ChiLibraryNode> {
    let action = Action { 
        id: Uuid::new_v4().to_string(), 
        method: String::from(""), 
        url: String::from(""), 
        name: String::from(""), 
        headers: Vec::new(), 
        query: Vec::new(), 
        path: Vec::new(), 
        body: String::from("") };

    let library = ChiLibraryNode { 
        id: Uuid::new_v4().to_string(), 
        name: String::from("New Library"),
        actions: vec![action], 
        children: Vec::new() 
    };
    let mut libraries = get_libs();
    libraries.push(library);
    update_libs(path, &libraries)
}

fn update_libs(path: String, libraries: &Vec<ChiLibraryNode>) -> Vec<ChiLibraryNode> {
    fs::write(path, format!("{:?}",libraries)).expect("Unable to write file");
    get_libs()
}

fn get_filestore() -> FileStore {
    let data = get_file_content(&get_filestore_path());
    serde_json::from_str::<FileStore>(data.as_str()).unwrap()
}

fn get_file_content(path: &String) -> String {
    let data = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => panic!("{:?}", error)
    };
    data
}

fn create_file(path: &String, contents: &str) -> String {
    fs::write(path, contents).expect("Unable to write file");
    get_file_content(path)
}

fn get_filestore_path() -> String {
    format!("{}/app.json", std::env::current_dir().unwrap().display().to_string())
}
