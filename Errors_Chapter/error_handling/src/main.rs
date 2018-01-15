use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //Rust has two ways of dealing with errors first a panic! macro that stops execution when run.
    //The other method is a Result<T,E> which allows one to continue running if an error is considered
    //recoverable.

    //The default behavior of panic! is for the program to start unwinding and cleaning up after itself
    //as it moves up the stack. This however takes a lot of work. The other alternative is to immediately
    //abort the program which then makes it the OS's resposibility for cleaning up the memory left behind.
    //You can add the below to your cargo.toml if you need your binary to be small and it needs to abort
    //[profile.release]
    //panic = 'abort'

    //Let's test the panic method.
    // panic!("crash and burn");
    //Sometimes our code might panic in a function that we call outside of our code. 
    //We would then need to use backtrace to find out where in our code led to the panic instead
    //of the exact location in the other code.

    // let v = vec![1, 2, 3];
    //One of the big plusses to Rust. It refuses to run if we try and access something out of bounds :D
    // v[100];
    
    //The Result enum is defined as having two variants Ok and Err as follows:
    //The T and E are generic type parameters
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //Since Result is generic we can use this type and functions in the standard
    //lib has defined on it in many different ways where the successful values
    //and errors we might want returned might differ.
    // If we assign this a type and compile the compiler will complain because
    //the type File::open is returning is different than the one we've given it.
    //This can also give us insight into what that type is.
    // let f: u32 = File::open("hello.txt");
    // This error told us Result is of type std::fs::File when it succeeds or
    // std::io:Error when it fails.

    //The below will fail but we can now process this as:
    // let f = File::open("hello.txt");
    //We can use the match expression to handle any errors we might have.
    // let f = match f {
    //     Ok(file) => file,
    //     //The File::open returns an error struct that is of io::Error which is provided
    //     //by the standard library
    //     //The below error tells us that the file does not exist.
    //     //The condition is called a match guard. It's an extra condition on the match
    //     //arm that further refines the arm pattern. This must be true for the code to
    //     //run. The ref before error ensures it does not move inside the guard condition
    //     //A ref matches a value and returns a refernce to it instead of & which matches
    //     // a reference and returns a value.
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt"){
    //             Ok(fc) => {
    //                 println!{"Created file."};
    //                 fc
    //                 }
    //             Err(e) =>{
    //                 panic!(
    //                     "Tried to create file but there was problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     //This last error causes the program to panic if any other error is encountered
    //     //other than the file not being found.
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}"
    //         , error
    //         )
    //     },
    // };

    //The match can also be a bit a verbose and Result<T,E> has a few helper functions
    //that can do several different tasks.
    //The unwrap method does just like the match but prints a generic panic message.
    // let f = File::open("hello.txt").unwrap();
    //If we want a more specific error message printed we can use:
    //which allows us to convey our intents much better. It also allows us to more
    //easily figure out where in our code the error occurred.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

use std::io;
use std::io::Read;
// use std::fs::File;

//This shows an example for how we can propogate errors out of a function.
//The header tells us if the function succeeds we will receive an Ok of type string.
//If it doesn't succeed will receive an Err of type io:Error because that is the type
//of error that we receive for each Err type in the function.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     //The read_to_string method reads the contents of the file and returns it as a string.
//     //This can also fail so we need another match here.
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

//Propogating errors is fairly common behavior in Rust and therefore a shortcut is 
//provided with the ? operator as seen below:
//It should be noted that the ? operator can only be used with functions that return
//a Result<T, E>. If the function doesn't then a compiler error will occur.
fn read_username_from_file() -> Result<String, io::Error> {
    //The ? operator when placed after result value works by passing an Ok value if
    //no problems exist, or it will return for the entire function the error message
    //that it received.
    //The difference between the ? method and the match method is that the ? method goes
    //through the from function in the From trait of the std lib. This from function
    //converts error types of one type into others as specifed by the function return type.
    //As long as each error type implements the from function to define how to convert itself
    //to the one given the ? operator takes care of the conversion automatically.

    //The ? operator also allows us to now chain code together and really shorten things
    //up. Here's the old way.
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
