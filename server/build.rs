// build.rs
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["../proto/user.proto"], &["../proto/"])?;
    Ok(())
}
