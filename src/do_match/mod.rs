/*Derive the environment by obtaining the environment variable PATH.*/
use std::{env, path::PathBuf};
pub fn init(){
match env::var("path").unwrap() {
    t if t.contains("apple") || t.contains("freebsd") || t.contains("openbsd") => {
        println!("cargo:rustc-link-lib=c++")
    }
    t if t.contains("msvc") => {
        print!("msvc env")
    }
    _ => println!("another env"),
}
}