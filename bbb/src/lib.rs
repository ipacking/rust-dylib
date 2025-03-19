#[unsafe(no_mangle)]
pub extern "Rust" fn add(a: i32, b: i32) -> i32 {
    a + b
}
