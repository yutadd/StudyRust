use study::ThreadPool;
use std::net::TcpListener;

fn init() -> ThreadPool {
    let th = ThreadPool::new(4);
    th
}

pub fn start() {
    /*simple html server*/
    let pool:ThreadPool = init();
    println!("[info]reated ThreadPool");
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    
    for stream in listener.incoming(){
        println!("[info]Connection Incoming");
        pool.execute(||{crate::lv2_handle_con::exec(stream.unwrap())});
    }
    print!("[info ]server has shutdown")
}
