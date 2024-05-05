use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/song.proto")?;

    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .type_attribute("Event", "#[derive(Hash)]")
        .compile(&["proto/song.proto"],&["proto"]);
    Ok(())
}