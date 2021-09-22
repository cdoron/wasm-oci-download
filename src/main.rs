use std::fs::File;
use std::io::Write;
mod oci_cache;

fn main() {
    let username = std::env::args().nth(1).expect("missing username");
    let password = std::env::args().nth(2).expect("missing password");
    let reference = std::env::args().nth(3).expect("missing image name");
    let data = oci_cache::cached_pull_wasm_module(username, password, reference).unwrap();
    let mut file = File::create("module.wasm").unwrap();
    file.write(&data);
}
