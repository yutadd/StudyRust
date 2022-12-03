struct ThreadPool;
impl ThreadPool {
   fn new(size: u32) -> ThreadPool { ThreadPool }
   fn execute<F>(&self, f: F)
       where F: FnOnce() + Send + 'static {}
}
pub fn init(){
    let pool = ThreadPool::new(4);
    pool.execute(move || {println!()});
}