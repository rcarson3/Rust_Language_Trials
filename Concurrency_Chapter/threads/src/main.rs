use std::thread;
use std::sync::mpsc;
use std::time::Duration;
//Rust provides a 1:1 thread model where however many threads we request corresponds
//to the number of OS threads. This is different from the M:N aka green thread model.
fn main() {
    //The code can prematurely get killed because the main thread dies before the spawned one finishes.
    //We can fix this by assigning the return type of thread::spawn to a variable.
    //The return type of a thread::spawn is a JointHandle. When we call the join method on it 
    //the handle will wait for its thread to finish its work.
    // let handle = thread::spawn(|| {
    //      for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    // }

    //We can move this around and leave both threads to not do work at the same time.
    // handle.join();


    let v = vec![1, 2, 3];
    //The move here is important because without it the spawned thread can't use the variable v.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    //If we uncomment the below the compiler will error at us because the main thread no longer
    //has ownership of the variable v.
    //drop(v);

    handle.join();

    //If we want to send data from thread to thread we need to use channels. You can
    //think of them as something similar to MPI send and recv calls in that they have
    //a send and recv end.

    //We create a new channel using the mpsc module also called multiple producer, single consumer.
    //A channel can have multiple sending ends but only a single receiving end.
    //This returns a tuple where tx is the transmitter and rx is the receiver.
    let (tx, rx) = mpsc::channel();

    //The transmitting end needs to own the transmitting end in order to be able to send
    //messages through a channel.
    thread::spawn(move || {
        let val = String::from("hi");
        //The send method takes the value that we want to send.
        //It returns a Result so we can check to see if the recv end has already been dropped.
        tx.send(val).unwrap();
    }); 

    //The recv method will block the main thread's execution and wait until a value is sent down the
    //channel. Once a value is sent recv will return a Result<T, E>/
    //The try_recv method is nonblocking and let's us do work while we wait for the tread
    //to recv a message if available. 
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //The below is an example showing how ownership works in rust with concurrency.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //This fails because we've moved ownership of the data to another thread.
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });


    let (tx, rx) = mpsc::channel();

    //Here we're cloning tx so that we can have multiple threads sending data to the main one. 
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //Here we can see that the main thread is waiting to receive the rest of the values that
    //are being sent over to it.
    for received in rx {
        println!("Got: {}", received);
    }

}
