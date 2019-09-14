use std::env;

pub fn main() {
    let devkitpro = env::var("DEVKITPRO").expect("is DEVKITPRO installed?");

    println!("cargo:rustc-link-lib=static=ogc");
    println!("cargo:rustc-link-search={}/libogc/lib/wii", devkitpro);
}
