use std::fs;

use crate::chi_models::ChiLibraryNode;

pub fn get_config() -> Vec<ChiLibraryNode> {
    let path: String = format!("{}/app.json", std::env::current_dir().unwrap().display());
    println!("{}", path);
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str::<Vec<ChiLibraryNode>>(data.as_str()).unwrap()
}

fn update_config(lib_id: &String, data: &String) {
    fs::write("./app.x77", data).expect("Unable to write file");
}
