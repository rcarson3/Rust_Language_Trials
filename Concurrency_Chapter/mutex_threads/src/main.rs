
//Rust allows concurrency by shared memory principles. Yay OpenMP equivalent.
//A mutex is a concurrency primative that allows for sharing of data. However,
//it only allows access to some data one thread at a time.
//If a mutex thread wants access to the data it needs to ask for the lock containing
//the data. Once your done with the data you must release the lock. From OpenMP type
//programming we know that this can really slow down the parallization of the program
//if it isn't done in a smart manner. For example if this is done in a loop you're
//going to take a big hit.

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {

    //m is of type Mutex<i32> in this case
    let m = Mutex::new(5);

    {
        //We need to aquire the lock in order to be able to use the data.
        //The lock call will block this thread from doing any work until the
        //lock is aquired. If another thread who was holding the lock panics then
        //we wouldn't be at a stand still. Therefore, we unwrap this call
        //to have it panic if that cases happens.
        let mut num = m.lock().unwrap();
        //The type returned from the lock is a smart pointer called MutexGuard.
        //We therefore need to dereference it in order to update the data that
        //we want to. Once the MutexGuard goes out of scope the lock is dropped.
        //This is nice because we don't have to remember to do this each time
        //we want to change something.
        *num = 6;
    }

    println!("m = {:?}", m);

    //The below example should fail when not used with Arc

    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        //This fails because we can't move ownership of counter into multiple threads
        //We also can't use Rc to fix this because it does not satisy safety requirements
        //to be used in concurrent problems. The reason behind this is that the way it
        //updates count is not done in a thread safe way as in another thread could
        //interrupt the process. We therefore need to use the Arc.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());    
    //The combined use of Arc and mutex can lead to deadlocks occurring in a code.

    //The rust language doesn't really have any concurrency built into the language.
    //Two concurrency types built into the language are the std:marker traits Sync and
    //Send.

    //The Send trait indicates that ownership of the type implementing this trait may be
    //transferred between threads. Most Rust types implement this trait. Any type that
    //is entirely composed of types marked with the Send trait is automatically marked
    //as Send as well. Almost all primitives implement Send except raw ptrs

    //The Sync trait indicates that it is safe for a type to be referenced from
    //multiple threads. One other way of saying this is that any type T is Sync if
    //&T is Send. Just like Send if all components of a type implement Sync it does
    //as well. Primitave types are Sync as well except for raw ptrs. Rc<T> and RefCell<T>
    //and it's other family members in Cell<T> do not implement Sync.

    //In order to manually implement Send and Sync traits one must use unsafe code.
     
}
