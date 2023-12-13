fn main() {
    println!("cargo:rerun-if-changed=goonto-res.rc");
    println!("cargo:rerun-if-changed=goonto.exe.manifest");

    #[cfg(all(not(debug_assertions), target_os = "windows"))]
    embed_resource::compile("goonto-res.rc", embed_resource::NONE);
}