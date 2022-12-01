mod another_file;
use std::io;
use std::thread;
use std::{env, path::PathBuf};
struct ThreadPool;
impl ThreadPool {
   fn new(size: u32) -> ThreadPool { ThreadPool }
   fn execute<F>(&self, f: F)
       where F: FnOnce() + Send + 'static {}
}
fn main() {
    /*This section does not use a thread pool so this will overflow some day.*/
    let mut user_input = String::new();
    print!("please input some text :");
    let ipt=io::stdin();
    ipt.read_line(&mut user_input).expect("stdin has error");
    println!("input text->{}",user_input);
    thread::spawn(move || another_file::init(&mut "not using pool".to_string()));
    
    let pool = ThreadPool::new(4);
    pool.execute(move || {another_file::init(&mut "using pool".to_string())});
/*Derive the environment by obtaining the environment variable PATH.*/
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