fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/*.proto");

    let protos = vec!["proto/healthcheck.proto", "proto/todo.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile(&protos, &["proto"])?;

    Ok(())
}
