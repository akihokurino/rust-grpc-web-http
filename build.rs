use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .compile(
            &[
                "proto/common.proto",
                "proto/user.proto",
                "proto/prefecture.proto",
            ],
            &["proto"],
        )
        .unwrap();
    Ok(())
}
