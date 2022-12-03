mod lv1_another_file;
mod lv1_do_match;
mod lv3_web_server;
mod lv2_using_thread_pool;
pub mod lv2_handle_con;
use std::io;
use std::thread;
/*@author 坂島悠太(DOTPIANO_DEV)
学習したことを追記していく*/

fn text_input(){
    let mut user_input = String::new();
    print!("please input some text :");
    let ipt=io::stdin();
    ipt.read_line(&mut user_input).expect("stdin has error");
    println!("\r\ninput text->{}",user_input);
}

fn main() {
    lv1_another_file::init(&mut "not using pool".to_string());
    /*This section does not use a thread pool so if this process is in loop this will overflow some day.*/
    let input_text_handle=thread::spawn(move || {text_input()});
    lv2_using_thread_pool::init();
    lv1_do_match::init();
    
    lv3_web_server::start();
    input_text_handle.join().expect("Thread join failed");
}