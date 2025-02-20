


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building...");
    tonic_build::compile_protos("protos/smollm.proto")?;
    Ok(())
}