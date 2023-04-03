use std::path::PathBuf;

/// Root directory of proto files
/// from where proto will compute `include`
const PROTO_ROOTS: &[&'static str] = &[concat!(
    env!(
        "CARGO_MANIFEST_DIR",
        "This is supposed to be run from cargo"
    ),
    "/protos"
)];

/// List of all proto files used in the project
const PROTO_FILES: &[&'static str] = &[
    concat!(
        env!(
            "CARGO_MANIFEST_DIR",
            "This is supposed to be run from cargo"
        ),
        "/protos/primitives.proto"
    ),
    concat!(
        env!(
            "CARGO_MANIFEST_DIR",
            "This is supposed to be run from cargo"
        ),
        "/protos/university.proto"
    ),
];

const PROTO_OUT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/protos");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let greeter_descriptor = out_dir.join("greeter_descriptor.bin");

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(greeter_descriptor)
        .build_client(true)
        .out_dir(PROTO_OUT_DIR)
        .compile(PROTO_FILES, PROTO_ROOTS)?;

    for proto_file in PROTO_FILES {
        println!("cargo:rerun-if-changed={}", proto_file);
    }

    Ok(())
}
