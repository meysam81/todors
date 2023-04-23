fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec!["proto/healthcheck.proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&protos, &["proto"])?;

    Ok(())
}
