fn main() {
    println!("cargo:rustc-link-search=native=../");
    println!("cargo:warning=BUILD.rs");
    println!("cargo:rustc-link-arg=-z max-page-size=4096");
}