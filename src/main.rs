use anyhow::Result;
use cached::proc_macro::cached;
use std::fs::File;
use std::io::Write;

use oci_distribution::{ manifest, secrets::RegistryAuth, Client};

// The size of the cache is 5.
// Entries stay for no more than an hour.
// Only `Ok` results are cached.
#[cached(size=5, time=3600, result=true)]
fn cached_pull_wasm_module(username: String, password: String, reference: String) -> Result<Vec<u8>> {
    return pull_wasm_module(username, password, reference);
}

#[tokio::main(flavor = "current_thread")]
async fn pull_wasm_module(username: String, password: String, reference: String) -> Result<Vec<u8>> {
    let reference = reference.parse()?;
    //dbg!(&reference);

    let mut client = Client::default();
    let registry_auth = RegistryAuth::Basic(username.parse()?, password.parse()?);
    let img = client
        .pull(
            &reference,
            &registry_auth,
            vec![
                manifest::WASM_LAYER_MEDIA_TYPE,
                manifest::IMAGE_MANIFEST_MEDIA_TYPE,
                manifest::IMAGE_DOCKER_LAYER_GZIP_MEDIA_TYPE,
            ],
        )
        .await?;

    println!("Downloaded {}", img.digest());

    let layer = img.layers.get(0).unwrap();
    return Ok(layer.data.clone());
}

fn main() {
    let username = std::env::args().nth(1).expect("missing username");
    let password = std::env::args().nth(2).expect("missing password");
    let reference = std::env::args().nth(3).expect("missing image name");
    let data = cached_pull_wasm_module(username, password, reference).unwrap();
    let mut file = File::create("module.wasm").unwrap();
    file.write(&data);
}
