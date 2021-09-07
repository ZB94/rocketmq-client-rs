extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let includes = env::var("ROCKETMQ_INCLUDES_DIR")
        .expect("请设置环境变量`ROCKETMQ_INCLUDES_DIR`, 值为ROCKETMQ-CLIENT-CPP的include目录");
    let libs = env::var("ROCKETMQ_LIBS_DIR")
        .expect("请设置环境变量`ROCKETMQ_LIBS_DIR`, 值为ROCKETMQ-CLIENT-CPP的静态库目录");

    println!("cargo:rustc-link-search={}", &libs);
    println!("cargo:rustc-link-lib=static=rocketmq");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=rt");

    println!("cargo:rerun-if-changed=wrapper.h");


    let bindings = bindgen::Builder::default()
        .clang_args(["-I", &includes])
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
