use std::fs;
use uuid::Uuid;

use crate::chi_models::{ChiLibraryNode, Action, ChiConfig};

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
    let data = app_data();
    serde_json::from_str::<Vec<ChiLibraryNode>>(data.as_str()).unwrap()
}

pub fn get_action(libraryId: Option<String>, actionId: Option<String>) -> Vec<ChiLibraryNode> {
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
    update_libs(&libraries)
}

fn update_libs(libraries: &Vec<ChiLibraryNode>) -> Vec<ChiLibraryNode> {
    fs::write(path(), format!("{:?}",libraries)).expect("Unable to write file");
    get_libs()
}

fn app_data() -> String {
    let data = match fs::read_to_string(path()) {
        Ok(content) => content,
        Err(_error) => create_app_data()
    };
    data
}

fn create_app_data() -> String {
    fs::write(path(), "[]").expect("Unable to write file");
    app_data()
}

fn path() -> String {
    format!("{}/app.json", std::env::current_dir().unwrap().display().to_string())
}
