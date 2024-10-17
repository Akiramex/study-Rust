use std::io::Result;

fn main() -> Result<()> {
    //tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("proto")
        .compile_protos(&["proto/voting.proto", "proto/hello.proto"], &["proto"])?;
    Ok(())
}