fn main() {
    // This file is necessary for compilation but doesn't need to do anything special
    println!("cargo:rerun-if-changed=build.rs");
}