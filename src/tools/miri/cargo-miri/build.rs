fn main() {
    // Don't rebuild miri when nothing changed.
    println!("cargo:rerun-if-changed=build.rs");
    // gather version info
    crablangc_tools_util::setup_version_info!();
}
