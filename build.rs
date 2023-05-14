#[cfg(feature = "sqlite")]
fn get_protos() -> Vec<&'static str> {
    vec!["proto/healthcheck.proto", "proto/todo-sqlite.proto"]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    println!("cargo:rerun-if-changed=proto/*.proto");

    let protos = get_protos();

    tonic_build::configure()
        .build_server(true)
        .compile_with_config(config, &protos, &["proto"])?;

    Ok(())
}
