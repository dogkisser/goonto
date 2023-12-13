fn main() {
    println!("cargo:rerun-if-changed=win/goonto-res.rc");
    println!("cargo:rerun-if-changed=win/goonto.exe.manifest");

    #[cfg(all(not(debug_assertions), target_os = "windows"))]
    embed_resource::compile("res/win/goonto-res.rc", embed_resource::NONE);
}