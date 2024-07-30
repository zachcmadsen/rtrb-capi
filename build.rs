use std::env;

fn main() {
    // rustc-cdylib-link-arg appends its argument to the linker invocation,
    // and -Wl passes comma-separated options to the linker. So, the code
    // below passes '-soname librtrb_capi.so.0' to the linker.
    // 
    // References:
    // - https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg
    // - https://gcc.gnu.org/onlinedocs/gcc/Link-Options.html
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,-soname,librtrb_capi.so.{}",
        env::var("CARGO_PKG_VERSION_MAJOR").unwrap()
    );
}
