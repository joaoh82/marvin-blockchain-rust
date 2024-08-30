fn main() {
    // println!("cargo:rerun-if-changed=src/proto/types.proto");
    prost_build::compile_protos(&["src/proto/types.proto"], &["src/proto"])
        .expect("Failed to compile proto");
}