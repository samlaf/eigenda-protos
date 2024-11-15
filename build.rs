fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &[
            "eigenda/api/proto/disperser/disperser.proto",
            "eigenda/api/proto/common/common.proto",
        ],
        &["eigenda/api/proto"],
    )?;
    Ok(())
}
