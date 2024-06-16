pub mod chat {
    tonic::include_proto!("chat");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../../proto/chat_descriptor.bin");
