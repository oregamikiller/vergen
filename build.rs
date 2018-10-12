extern crate chrono;

pub fn main() {
    use chrono::prelude::*;
    let now = UTC::now();
    println!(
        "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}",
        now.to_rfc3339()
    );
}
