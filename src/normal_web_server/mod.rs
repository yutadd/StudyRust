use std::net::TcpListener;

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
mod handle_connection;
trait FnBox{
    fn call_box(self:Box<Self>);
}
impl<F:FnOnce()> FnBox for F{
    fn call_box(self:Box<F>) {
        (*self)();
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread=thread::spawn(move ||{
            loop{
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
    
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
        }
    });
    Worker {
        id,
        thread: Some(thread),
    }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
 impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver)=mpsc::channel();
        let receiver:Arc<Mutex<mpsc::Receiver<Job>>>=Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
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
        pool.execute(||{handle_connection::exec(stream.unwrap())});
    }
    
}
