fn main() {
    prost_build::compile_protos(&["src/gtfs.proto"], &["src/"]).unwrap();
}
