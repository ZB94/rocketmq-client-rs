# RocketMQ-Client-rs

本库是对 [RocketMQ-Client-cpp](https://github.com/apache/rocketmq-client-cpp) CAPI的封装

使用之前需要先自行编译`RocketMQ-Client-cpp`

**注意：**

本库目前只是对接口的简易包装，未进行过测试。

## Features

- **generated**(默认)

    使用`RocketMQ-Client-cpp`的 [release-2.2.0](https://github.com/apache/rocketmq-client-cpp/tree/release-2.2.0) 头文件生成

- **autogen**

    与`generated`功能相同，但内容为编译时动态生成

## 编译时环境变量

- **ROCKETMQ_LIBS_DIR**

  指向`RocketMQ-Client-cpp`编译后生成的静态库所在的目录

- **ROCKETMQ_INCLUDES_DIR**(非必须)

  指向`RocketMQ-Client-cpp`的头文件目录，仅在启用`autogen`时需要设置
