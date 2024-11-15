fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from("src/generated");
    std::fs::create_dir_all(&out_dir)?;

    tonic_build::configure().out_dir(&out_dir).compile_protos(
        &[
            "eigenda/api/proto/disperser/disperser.proto",
            "eigenda/api/proto/common/common.proto",
        ],
        &["eigenda/api/proto"],
    )?;
    Ok(())
}
