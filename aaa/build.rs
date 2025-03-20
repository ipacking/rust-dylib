fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-search=native=target/debug");
        println!("cargo:rustc-link-lib=dylib=bbb.dll");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-search=native=target/debug");
        println!("cargo:rustc-link-lib=dylib=bbb");
    }
}
