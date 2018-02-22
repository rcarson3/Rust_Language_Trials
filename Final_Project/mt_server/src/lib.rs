use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//We're doing a little bit of refactoring to have a vector of workers which has an
//id and a JoinHandle. It will also be what handles the closure
//We want our workers to either fetch a job or terminate depending on what message is being sent to it.
//This is to deal with the infinit loop inside of the workers closure.
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
//The below is how to do a doc comment
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    pub fn new(size: usize) -> ThreadPool {
        //We need more than 0 threads
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        //Arc allows multiple workers to own the receiver and
        //Mutex ensures that only one worker is getting a job from
        //the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            //We're going to create some threads here and then store them in the above vector.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender,
        }
    }

    //We're going to eventually want to pass the arguments from this into 
    //thread::spawn so we want to make sure we use the same function types as that method.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
//We never properly cleaned up our data once we quit out of everything just using Ctrl-C.
//So we are going to implement the Drop command to do this for us.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        //Here we're telling all of the workers to terminate.
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            //We needed a way to take ownership of thread out of the worker.
            //So we use the take option of Option.
            //This will then replace the thread variable inside of worker with
            // a None type.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


//We need some form of actual closures when we create the workers
//Earlier we learned about channels. Here we'll be using them. We'll have the thread pool
//hold the jobs and then have the workers fetch the jobs.

//A job struct will hold the closures we want to send down the channel.
//The execute method on the thread pool will send the jobs down the sending side of the channel
//Workers will then loop over their receiving side of the channel and execute the closures of
//any jobs they receive.

//In order to get around rust not realizing we can do self: Box<Self>
//we're going to explicitly tell it that we are taking ownership of the closure
//inside Box and use it. 
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

//We've changed Job from a struct to a type alias to make
//our long type just a little shorter
//The Job now just needs to be updated to use the FnBox trait.
type Job = Box<FnBox + Send + 'static>;

struct Worker {
    id: usize,
    //thread::spawn returns a JoinHandle<T>
    //So we probably want a JoinHandle to hold our
    //thread for our pool.
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        //Our worker is now receiving a reference to a closure
        //Remember to include the semicolon inside the closure...
        //We need to loop the closure forever asking the receiver end for a job.
        //Then we run the job once we get it.
        let thread = thread::spawn(move || {
            loop{
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        //Rust currently doesn't realize that it can use self: Box<Self>  to unpack the closure
                        //and move it outside of Box<T> so we can actually use it.
                        //See the above FnBox trait for how to get around this.
                        // (*job)();
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        //Here if we get a terminate message the loop is broken out of.
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