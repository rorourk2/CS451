use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
//(sender, receiver)=mpsc::channel();

enum Message{
	NewJob(Job),
	Shutdown,
}

pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
	//sender: mpsc::Sender<Job>,
}

impl Drop for ThreadPool{
	fn drop(&mut self){
	
		for worker in &mut self.workers {
			//eprintln!("Sending shutdown message to worker {}", worker.id);
			self.sender.send(Message::Shutdown).unwrap();
		}

		for worker in &mut self.workers {
			//eprintln!("Shutting down worker {}", worker.id);
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
			
		}
		
	}

}


struct Worker {
	id: usize,
	//thread: thread::<thread::JoinHandle<()>>,
	thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self:Box<F>){
		(*self)();
	}
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool{

		assert!(size > 0);

		let(sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		//let mut threads = Vec::with_capacity(size);
		let mut workers = Vec::with_capacity(size);

		for id in 0..size{
			//thread::spawn(||{})
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool{
			workers,
			sender,
		}
	}

	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		//self.sender.send(job).unwrap();
		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

/*impl Drop for ThreadPool {
	fn drop(&mut self){
	
	}
}*/

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
		let thread = thread::spawn(move || {
			loop{
				let message=receiver.lock().unwrap().recv().unwrap();
				//eprintln!("Worker {} got a job", id);
				match message{
					Message::NewJob(job) =>{
						//eprintln!("Worker {} got a job.", id);
						job.call_box();
					}
					Message::Shutdown => {
						//eprintln!("Worker {} shutting down", id);
						break;
					}
				};
			}
		});

		let thread = Some(thread);

		Worker{
			id,
			thread,
		}
	}
}