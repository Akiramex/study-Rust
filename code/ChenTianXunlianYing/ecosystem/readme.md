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