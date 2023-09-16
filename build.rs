extern crate cc;
extern crate cmake;

fn main() {
    println!("cargo:rustc-link-lib=static=box2d");
    println!("cargo:rerun-if-changed=box2d/");
    println!("cargo:rerun-if-changed=frontend/");
    let mut include_dir: String = String::new();
    if let Some(path) = std::env::var("BOX2D_LIB_DIR").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    } else {
        let box2d_install_prefix = cmake::Config::new("box2d")
            .define("BOX2D_BUILD_UNIT_TESTS", "OFF")
            .define("BOX2D_BUILD_TESTBED", "OFF")
            .define("BOX2D_BUILD_DOCS", "OFF")
            .define("BOX2D_USER_SETTINGS", "OFF")
            .define("BUILD_SHARED_LIBS", "OFF")
            .build();
        println!(
            "cargo:rustc-link-search=native={}/lib",
            box2d_install_prefix.display()
        );
        include_dir = format!("{}/include", box2d_install_prefix.display());
        println!("cargo:include={}/include", box2d_install_prefix.display());
    };

    cc::Build::new()
        .cpp(true)
        .include(include_dir)
        .file("frontend/lib.cpp")
        .std("c++11")
        .compile("libbox2d_frontend.a");
}
