extern crate remote_trait_object_tests;

#[cfg(all(unix, target_arch = "x86_64"))]
fn main() -> Result<(), String> {
    let args = std::env::args().collect();
    remote_trait_object_tests::mod_hello_main(args);
    Ok(())
}
