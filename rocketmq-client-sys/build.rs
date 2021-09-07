use std::env;

fn main() {
    let libs = env::var("ROCKETMQ_LIBS_DIR")
        .expect("请设置环境变量`ROCKETMQ_LIBS_DIR`, 值为ROCKETMQ-CLIENT-CPP的静态库目录");

    println!("cargo:rustc-link-search={}", &libs);
    println!("cargo:rustc-link-lib=static=rocketmq");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=rt");
}
