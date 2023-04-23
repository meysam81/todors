fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec!["proto/greet.proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&protos, &["proto"])?;

    Ok(())
}
