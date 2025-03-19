fn main() {
    let sum = unsafe { add(1, 2) };
    println!("sum: {}", sum);
}

unsafe extern "Rust" {
    unsafe fn add(a: i32, b: i32) -> i32;
}
