#[cfg(feature = "generate")]
extern crate bindgen;

use std::env;

fn main() {
    #[cfg(feature = "generate")] {
        use std::path::PathBuf;

        let includes = env::var("ROCKETMQ_INCLUDES_DIR")
            .expect("请设置环境变量`ROCKETMQ_INCLUDES_DIR`, 值为ROCKETMQ-CLIENT-CPP的include目录");
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

    let libs = env::var("ROCKETMQ_LIBS_DIR")
        .expect("请设置环境变量`ROCKETMQ_LIBS_DIR`, 值为ROCKETMQ-CLIENT-CPP的静态库目录");
    println!("cargo:rustc-link-search={}", &libs);
    println!("cargo:rustc-link-lib=static=rocketmq");

    if cfg!(feature = "musl-static") {
        let musl_libs = env::var("MUSL_LIBS_DIR")
            .expect("请设置环境变量`MUSL_LIBS_DIR`, 值为musl工具链中静态库的目录(通常在`工具链目录/x86_64-linux-musl/lib/`)");

        println!("cargo:rustc-link-search={}", musl_libs);
        println!("cargo:rustc-link-lib=static=Signature");
        println!("cargo:rustc-link-lib=static=boost_atomic");
        println!("cargo:rustc-link-lib=static=boost_chrono");
        println!("cargo:rustc-link-lib=static=boost_date_time");
        println!("cargo:rustc-link-lib=static=boost_filesystem");
        println!("cargo:rustc-link-lib=static=boost_iostreams");
        println!("cargo:rustc-link-lib=static=boost_locale");
        println!("cargo:rustc-link-lib=static=boost_log");
        println!("cargo:rustc-link-lib=static=boost_log_setup");
        println!("cargo:rustc-link-lib=static=boost_regex");
        println!("cargo:rustc-link-lib=static=boost_serialization");
        println!("cargo:rustc-link-lib=static=boost_system");
        println!("cargo:rustc-link-lib=static=boost_thread");
        println!("cargo:rustc-link-lib=static=boost_wserialization");
        // println!("cargo:rustc-link-lib=static=boost_zlib");
        println!("cargo:rustc-link-lib=static=bz2");
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=event");
        println!("cargo:rustc-link-lib=static=event_core");
        println!("cargo:rustc-link-lib=static=event_extra");
        println!("cargo:rustc-link-lib=static=event_openssl");
        println!("cargo:rustc-link-lib=static=event_pthreads");
        println!("cargo:rustc-link-lib=static=jsoncpp");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=static=pthread");
        println!("cargo:rustc-link-lib=static=dl");
        println!("cargo:rustc-link-lib=static=rt");
        println!("cargo:rustc-link-lib=static=stdc++");
    } else {
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=rt");
        println!("cargo:rustc-link-lib=stdc++");
    }
}
