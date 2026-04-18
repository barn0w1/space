// build.rs
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["../proto/event.proto"], &["../proto/"])?;
    Ok(())
}
