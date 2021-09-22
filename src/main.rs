mod oci_cache;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transformation {
    name: String,
    username: String,
    password: String,
    wasm_image: String,
}

fn load_single_transformation(entity: serde_yaml::Value) {
    let t: Transformation = serde_yaml::from_value(entity).unwrap();
    println!("Trying to obtain WASM Module: {}", t.name);
    let _wasm_binary = oci_cache::cached_pull_wasm_module(t.username, t.password, t.wasm_image);
 }

fn traverse_transformations(s: String) {
    let data: serde_yaml::Value = serde_yaml::from_str(&s).unwrap();
    let yaml_serde_sequence = data["transformations"].as_sequence().unwrap();
    for (_, entity) in yaml_serde_sequence.iter().enumerate() {
        // is there a way to send entity and not its clone ???
        load_single_transformation(entity.clone());
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("missing yaml filename");
    let s = std::fs::read_to_string(filename).unwrap();
    traverse_transformations(s);
}
