use std::io::Result;

fn main() -> Result<()> {
    //tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("proto")   //输出目录名
        // 输入文件 & 包含的路径
        .compile_protos(&["proto/voting.proto", "proto/greet.proto"], &["proto"])?;
    Ok(())
}