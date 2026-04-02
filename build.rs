fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-env-changed=INVENTORY_REPORT_URL");

    let inv_url = std::env::var("INVENTORY_REPORT_URL").unwrap_or_default();
    let inv_path = std::path::Path::new(&out).join("default_inventory_report_url.rs");
    let inv_src = if inv_url.is_empty() {
        "pub const DEFAULT_INVENTORY_REPORT_URL_FROM_BUILD: &str = \"\";\n".to_string()
    } else {
        format!(
            "pub const DEFAULT_INVENTORY_REPORT_URL_FROM_BUILD: &str = {:?};\n",
            inv_url
        )
    };
    std::fs::write(&inv_path, inv_src).expect("write default_inventory_report_url.rs");

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
