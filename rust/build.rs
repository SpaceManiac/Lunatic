extern crate cpp_build;

fn main() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = root.parent().unwrap();
    cpp_build::Config::new()
        .include(root.join("include"))
        .include(root.join("source"))
        .define("ALLEGRO_MINGW32", None)
        .build("lib.rs");
}
