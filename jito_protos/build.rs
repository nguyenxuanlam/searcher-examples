use tonic_build::configure;

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("protos");

    let proto_files = [
        "auth.proto",
        "block.proto",
        "block_engine.proto",
        "bundle.proto",
        "packet.proto",
        "relayer.proto",
        "searcher.proto",
        "shared.proto",
    ];

    // Ensure proto directory exists
    std::fs::create_dir_all(&proto_dir)?;

    // Create full paths to proto files
    let proto_paths: Vec<PathBuf> = proto_files
        .iter()
        .map(|f| proto_dir.join(f))
        .collect();

    // Print the paths for debugging
    for path in &proto_paths {
        println!("cargo:warning=Looking for proto file at: {}", path.display());
    }

    tonic_build::configure()
        .build_server(true)
        .compile(
            &proto_paths
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>(),
            &[proto_dir],
        )?;

    // Rerun if any proto file changes
    for proto_file in proto_files.iter() {
        println!("cargo:rerun-if-changed=protos/{}", proto_file);
    }

    Ok(())
}
