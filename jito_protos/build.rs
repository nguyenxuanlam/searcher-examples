use tonic_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "protos";
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

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto") // Optional: output to a specific directory
        .compile(
            &proto_files.iter().map(|f| format!("{}/{}", proto_dir, f)).collect::<Vec<_>>(),
            &[proto_dir],
        )?;

    // Tell cargo to rerun this build script if the proto files change
    for proto_file in proto_files.iter() {
        println!("cargo:rerun-if-changed=protos/{}", proto_file);
    }

    Ok(())
}
