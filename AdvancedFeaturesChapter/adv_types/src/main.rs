//The newtype pattern can be used for type safety and abstraction
//In the adv_traits section we had Millimeters and Meters types that wrapped
//a u32 type. By doing this we can ensure that we won't accidentally use one for the
//other. If we do it the compiler will error on us. This is awesome for things were
//units are incredibly important and we can implement them into the type system where
//we can make sure we don't have something like the Mars Climate Orbiter mission.

//They can also be used to abstract away certain implementations of an inner type that we don't
//want don't want the public to mess with.

//We can also create type aliases which gives an existing type another name.
//Type aliases are the exact same as the existing type and not viewed as a
//seperate type as a newtype would create. Therefore, we can see down below that
//we can add an i32 and a kilometer together.
type Kilometers = i32;

//They are useful when we want to reduce repetition of having to write out a lengthy type
//all over the place. When doing this we might also make a mistake somewhere.

use std::io::Error;
use std::fmt;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// type Result<T> = Result<T, std::io::Error>;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
// }

//Rust has a special type called the Never Type, !, that never returns. Functions that
//return never are called diverging functions.

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             The panic!() is a never type because it doesn't produce a value; it ends the program.
//             In the None case we won't be returning a value so this code is valid.
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

// continue is also of type never because it moves control back to the top of the loop.
// let guess: u32 = match guess.trim().parse() {
//     Ok(num) => num,
//     Err(_) => continue,
// };

// A loop is also a ! type because it continues on forever. If it did have a break in it then it wouldn't be of type !.
// print!("forever ");

// loop {
//     print!("and ever ");
// }

//Dynamically sized types (DSTs) &sized
//DSTs are types whose size we only know at runtime. In general with DSTs they have references with extra bits of
//metadata attached like in the case of str - &str contains a pointer and the size of the data. The golden rule for
//DSTs is that we must always put values of DSTs behind a pointer of some kind. Every trait is a dynamically sized
//type. When we were talking about trait objects we mentioned that they have to be placed behind a &Trait or Box<Trait> or Rc<Trait>,
//so now we can see the reason why.

//In order to work with DSTs, Rust has a trait that determines if a type's size is known at compile time called Size. 
//This trait is automatically implemented for everything the compiler knows the size of at compile time.

// fn generic<T>(t: T) {
//     // ...snip...
// }
//At compile time the compiler automatically adds the following to the generic function.
// fn generic<T: Sized>(t: T) {
//     // ...snip...
// }
// If the generic function doesn't have a known size at compile time we can relax the restriction of a generic
//function requiring a known type size with ?Sized. This pretty much just says a type may or may not have a known size.
//Also note that t: T became t: &T
//This is because of the golden rule once again that we need to have the type behind some sort of pointer.
// fn generic<T: ?Sized>(t: &T) {
//     // ...snip...
// }

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //The following could be error prone so we're going to replace it.
    // let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    //     // ...snip...
    // }

    // fn returns_long_type() -> Box<Fn() + Send + 'static> {
    //     // ...snip...
    // }
    //A good thing to do for type aliasing is just picking an appropriate name that
    //that conveys the meaning of what your replacing it with.
    // type Thunk = Box<Fn() + Send + 'static>;

    // let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Thunk) {
    //     // ...snip...
    // }

    // fn returns_long_type() -> Thunk {
    //     // ...snip...
    // }
}
