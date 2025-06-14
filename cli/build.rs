fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tonic_build::configure()
        .build_client(true)
        .compile(
            &["../proto/vehicle-shadow/signal.proto"],
            &["../proto"],
        )?;
    Ok(())
} 