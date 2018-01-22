
//An example of a list that doesn't work in Rust due to its recursive
//nature.
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
use std::ops::Deref;
//Box<T> is defined as a tuple struct with just one element.
struct MyBox<T>(T);
//Here we can see that the new operator takes the data and just encapsulates it
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//The Deref trait, provided by the standard library, requires implementing
// one method named deref that borrows self and returns a reference to
// the inner data.


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

//The second trait important to smart pointers is the drop trait. It
//lets us customize what happens when a value goes out of scope.
//The compiler will automatically place these calls for us so that
//we don't have to worry about manual memory management unless we want to.
//The Drop trait requires us to implement one method named drop that takes
//a mut ref to self. 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

//std::mem::drop; is in the prelude so we don't need to call it.
fn main() {
    //Box allows us to point to data being stored on the heap.
    //They also have several uses like when you want to transfer ownership
    //of data without it being copied,
    //Other cases are like when you have a type whose size isn't known
    //at compile time and you want use a value of that type in context
    //that needs to know an exact size.
    let b = Box::new(5);
    println!("b = {}", b);
    //When a box pointer goes out of scope both it and the data it's
    //pointing to go out of scope.

    //Boxes can also be used to define recursive types.
    //The below won't work because the compiler can't figure out
    //how much space this structure will require to be held.
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    //The Box pointer has therefore broken the infinite recursion cycle,
    //and the compiler can now figure out the size of Cons.

    //The * operator allows us to follow a reference to the value it
    //is point at. Box types implement this feature called the
    //Deref type.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //We can implement our version of the Box pointer that allows
    //us to dereference the data
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Derefence coercion is a convience that Rust performs on args of
    //fcns and methods that converts a reference to a type that implements
    //Deref into a ref to a type that Deref can convert the original type
    //to. This featue was added as a convience to reduce the number
    //of & and * needed.
    let m = MyBox::new(String::from("Rust"));
    //This deref is all run at compile time so we don't have performance
    //penalty during runtime.
    hello(&m);
    // Rust does deref coercion when it finds types and trait
    //  implementations in three cases:
// 
    // From &T to &U when T: Deref<Target=U>.
    // From &mut T to &mut U when T: DerefMut<Target=U>.
    // From &mut T to &U when T: Deref<Target=U>.
    //The reverse of the last point can't occur because you can't
    //convert an immutable ref to mut ref because there's no
    //guarantee that there is only one immutable ref to the data.

    //The below example is a visual example that lets us see how Rust
    //automatically manages the memory for us.
    // let c = CustomSmartPointer { data: String::from("my stuff") };
    // let d = CustomSmartPointer { data: String::from("other stuff") };
    // println!("CustomSmartPointers created.");

    //Here we're going to see that we can drop this pointer like its
    //hot. In other words we're going to deallocate everything
    //before our program is done because we need our space back.
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}
