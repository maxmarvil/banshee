use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/song.proto")?;

    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .type_attribute("Event", "#[derive(Hash, serde::Deserialize)]")
        .out_dir("src/grpc")
        .compile(&["proto/song.proto"],&["proto"]);
    println!("cargo:rerun-if-changed={}", "src/grpc");
    Ok(())
}