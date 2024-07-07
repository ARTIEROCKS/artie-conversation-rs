pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../../proto/chat_descriptor.bin");
tonic::include_proto!("chat");