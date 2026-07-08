fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-env-changed=INVENTORY_REPORT_URL");
    println!("cargo:rerun-if-env-changed=RUSTDESK_APP_NAME");
    println!("cargo:rerun-if-env-changed=RUSTDESK_PRESET_PASSWORD");

    let inv_url = std::env::var("INVENTORY_REPORT_URL").unwrap_or_default();
    let app_name =
        std::env::var("RUSTDESK_APP_NAME").unwrap_or_else(|_| "TnursRemoteDesk".to_string());
    let preset_password = std::env::var("RUSTDESK_PRESET_PASSWORD")
        .unwrap_or_else(|_| "91xd32mifk7fbz".to_string());
    let build_defaults_path = std::path::Path::new(&out).join("build_defaults.rs");
    let build_defaults_src = format!(
        "pub const DEFAULT_INVENTORY_REPORT_URL_FROM_BUILD: &str = {inv_url:?};\n\
pub const DEFAULT_APP_NAME_FROM_BUILD: &str = {app_name:?};\n\
pub const DEFAULT_PRESET_PASSWORD_FROM_BUILD: &str = {preset_password:?};\n"
    );
    std::fs::write(&build_defaults_path, build_defaults_src).expect("write build_defaults.rs");

    let out_dir = format!("{out}/protos");

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
