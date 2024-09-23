use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env!("IS_DOCKER") == "true" {
        configure("./api.proto", false, true)?;
    } else {
        configure("../usernator/api.proto", false, true)?;
    }
    configure("./tasky.proto", true, false)?;
    Ok(())
}

fn configure(
    proto: impl AsRef<Path>,
    server: bool,
    client: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let proto_path: &Path = proto.as_ref();
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .build_server(server)
        .build_client(client)
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
