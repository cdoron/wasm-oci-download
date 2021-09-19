#RUST_LOG=oci_distribution=trace cargo run --example download-image -- webassembly.azurecr.io/hello-wasm
RUST_LOG=oci_distribution=trace cargo run --example download-image -- the-mesh-for-data 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 ghcr.io/the-mesh-for-data/krustlet-tutorial:v1.0.0
