// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     //The reference counted smart pointer Rc<T> allows us to have 
//     //multiple ownerships of some data. You can think of it being useful
//     //in either a tree or persistent type data structure.
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     //When we clone a Rc pointer it increases the number of references
//     //that have access the data. In the below when b is created
//     //the reference to a is increased from 1 to 2.
//     //Rc::Clone also doesn't perform a deep copy of the data but instead
//     //just increments the Rc.
//     let b = Cons(3, Rc::clone(&a));
//     //Rc::strong_count gives us the number of Rc to a Rc pointer
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     //Rc pointers are immutable.
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
//Combining both Rc and RefCell we can have data that has multiple
//owners and that can be mutated.
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    //Here we can see the deref trait being used so that we can
    //access our initial RefCell which we can then borrow and mutate.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}