fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the vendored protoc binary
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    let protos = [
        "proto/aegis_runtime.proto",
        "proto/smcp_gateway.proto",
        "proto/aegis_cluster.proto",
        "proto/embedding.proto",
        "proto/aegis_remote_storage.proto",
        "proto/aegis_cortex.proto",
    ];

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&protos, &["proto"])?;

    Ok(())
}
