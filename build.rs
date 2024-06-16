fn main() {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("proto/chat_descriptor.bin") 
        .compile(&["proto/chat.proto"], &["proto"])
        .unwrap();
}
