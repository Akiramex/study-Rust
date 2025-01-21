## 错误处理

anyhow

thiserror

## 日志处理

tracing

写到文件用tracing_appender，但是输出文件有无意义乱码

：原来是ansi颜色代码，with_ansi(false)即可

## 宏

derive_builder: 构建数据结构的 builder

derive_more: 标准库trait的自动实现

strum: enum相关的trait的自动实现

## 序列化/反序列化

serde



~~~ Rust
// 重命名序列化后的名字形式
#[serde(rename_all = "camelCase")]

// 重命名单个字段
#[serde(rename = "rename")]

// 自定义序列化/反序列化函数
#[serde(serialize_with = "fn_serialize", deserialize_with = "fn_deserialize")]

// 跳过某个字段
#[serde(skip)]

// 如果fn为ture，跳过某个字段
#[serde(skip_serializing_if = "fn")]

// 如果反序列化没有值，就调用defalut
#[serde(default)]
#[serde(default = "fn")]
~~~

serde_with

看doc，DisplayFromStr， FromStr序列化，Display展示反序列化

## tokio

tokio生态

### Bytes

更好的字节处理库