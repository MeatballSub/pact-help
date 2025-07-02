use std::path::PathBuf;

fn main() {
    let out = PathBuf::from("../generated/rust");
    let descriptor_file = out.join("service_descriptors.bin");
    let proto_dir = PathBuf::from("../../proto");
    let proto_file = proto_dir.join("list_appender.proto");

    tonic_build::configure()
        .message_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .message_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .message_attribute(".", "#[serde(default)]")
        .enum_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .file_descriptor_set_path(&descriptor_file)
        .out_dir(out)
        .compile_protos(&[proto_file], &[proto_dir])
        .unwrap();
}
