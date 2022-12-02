/*use std::thread;
use std::time::Duration;*/
pub fn init(message: &mut str) {
    //thread::sleep(Duration::from_millis(500));
    let it = message.trim().chars();
    for ch in it {
        println!("yeah->{:#?}", ch);
    }
}
