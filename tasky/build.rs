use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env!("IS_DOCKER") == "true" {
        return configure("./api.proto");
    } else {
        return configure("../usernator/api.proto");
    }
}

fn configure(proto: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let proto_path: &Path = proto.as_ref();
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .build_server(false)
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
