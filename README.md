# RocketMQ-Client-rs

本库是对 [RocketMQ-Client-cpp](https://github.com/apache/rocketmq-client-cpp) CAPI的封装

使用之前需要先自行编译`RocketMQ-Client-cpp`

**注意：**

- 本库目前只是对接口的简易包装，使用不当可能会出现内存泄漏。
- 本库目前仅在**linux 64位**环境下使用过，其他系统使用前需自行测试。

## Features

- **default**

    使用`RocketMQ-Client-cpp`的 [release-2.2.0](https://github.com/apache/rocketmq-client-cpp/tree/release-2.2.0) 头文件生成的`sys`库

- **generate**

    使用指定的头文件重新生成`sys`库。
    
    编译时需要设置环境变量`ROCKETMQ_INCLUDES_DIR`，具体说明见[编译时环境变量](##编译时环境变量)

- **musl-static**

  使用`musl`的工具链进行静态链接。使用的工具链和编译的静态库存放于项目的`musl-static`目录，如果使用自己编译的静态库可能需要修改`sys`库中的链接参数。

  使用该`feature`编译时 Rust 需添加对应的`target`（如`x86_64-unknown-linux-musl`)并设置好对应`target`的链接工具路径(即在Cargo配置文件中为对应的`target`设置`linker`参数)，并在编译时指定`target`。

  编译时需要设置环境变量`MUSL_LIBS_DIR`，具体说明见[编译时环境变量](##编译时环境变量)

## 编译时环境变量

- **ROCKETMQ_LIBS_DIR**

  指向`RocketMQ-Client-cpp`编译后生成的静态库所在的目录

- **ROCKETMQ_INCLUDES_DIR** `generate` 

  指向`RocketMQ-Client-cpp`的头文件目录，仅在启用`autogen`时需要设置

- **MUSL_LIBS_DIR** `musl-static`

  工具链中静态库的目录，如`musl-static`中的工具链，值应为`工具链目录/x86_64-linux-musl/lib/`。
