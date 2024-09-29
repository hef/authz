fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::configure()
    .compile_protos(&[
        "proto/envoy/service/auth/v3/external_auth.proto"
    ], &["proto"])?;
    Ok(())
}