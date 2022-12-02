use std::net::TcpListener;

use std::sync::Arc;
use std::sync::Mutex;

use std::thread;
use std::sync::mpsc;
mod handle_connection;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(id: usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
type Job= Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender:mpsc::Sender<Job>,
}
 impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver)=mpsc::channel();
        let receiver:Arc<Mutex<mpsc::Receiver<Job>>>=Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i,receiver.clone()));
        }
        ThreadPool {  workers,sender }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
fn init() -> ThreadPool {
    let th = ThreadPool::new(4);
    th
}

pub fn start() {
    /*simple html server*/
    let pool:ThreadPool = init();
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    
    for stream in listener.incoming(){
        pool.execute(||{handle_connection::exec(stream.unwrap())});
    }
    
}
