fn main() {
    prost_build::compile_protos(&["protobuf/types.proto", "protobuf/messages.proto"],
                                &["protobuf/"]).unwrap();
}
