// build.rs
fn main() {
    tonic_build::compile_protos("proto/chat.proto").unwrap();
}
