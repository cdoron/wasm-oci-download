use anyhow::Result;
use cached::proc_macro::cached;
use oci_distribution::{ manifest, secrets::RegistryAuth, Client};
use std::env;

// The size of the cache is 5.
// Entries stay for no more than an hour.
// Only `Ok` results are cached.
#[cached(size=5, time=3600, result=true)]
pub fn cached_pull_wasm_module(credentials_variable: Option<String>, reference: String) -> Result<Vec<u8>> {
    return pull_wasm_module(credentials_variable, reference);
}

fn get_registry_auth(credentials_variable: Option<String>, reference: &String) -> RegistryAuth {
    if credentials_variable.is_some() {
        let credentials = env::var(credentials_variable.unwrap());
        if credentials.is_ok() {
            let credentials_json = credentials.unwrap();
            let data: serde_json::Value = serde_json::from_str(&credentials_json).unwrap();
            for (_, value) in data["auths"].as_object().unwrap() {
                println!("YYY {}", reference);
                let username = (value.as_object().unwrap()).get("username").unwrap().to_string();
                let password = (value.as_object().unwrap()).get("password").unwrap().to_string();
                println!("QQQ {}: MOO {}", username, password);
            }
        }
    }
    RegistryAuth::Anonymous
}

#[tokio::main(flavor = "current_thread")]
async fn pull_wasm_module(credentials_variable: Option<String>, reference: String) -> Result<Vec<u8>> {
    let registry_auth: RegistryAuth = get_registry_auth(credentials_variable, &reference);
    let reference = reference.parse()?;
    let mut client = Client::default();

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