use anyhow::Result;
use std::fs::File;
use std::io::Write;

use oci_distribution::{ manifest, secrets::RegistryAuth, Client};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();

    let username = std::env::args().nth(1).expect("missing username");
    let password = std::env::args().nth(2).expect("missing password");
    let reference = std::env::args().nth(3).expect("missing image name");
    let reference = reference.parse()?;
    dbg!(&reference);

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
    let data = layer.data.clone();
    let mut file = File::create("module.wasm")?;
    file.write(&data);
    Ok(())
}
