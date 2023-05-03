fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    println!("cargo:rerun-if-changed=proto/*.proto");

    let protos = vec!["proto/healthcheck.proto", "proto/todo.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile_with_config(config, &protos, &["proto"])?;

    Ok(())
}
