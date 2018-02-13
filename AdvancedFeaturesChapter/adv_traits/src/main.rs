
//Associated types are a way of associating a type placeholder with a trait such that the
//trait method definitions can use these placeholder types in their signatures.

//This tells us that the Iterator trait has an associated type named Item.
//Item is a placeholder type, and the return value of the next method is of type
//Option<Self::Item>. The actual implementation will specify the type and the return
//value will be of the type specified.

#![allow(unused_variables)]
#![allow(dead_code)]
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
//The below is from main.rs in the Functional_Chapter/iterators/src/
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    //Here we're just specifying that the returned item is of type u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        //We only want this to iterate from 1 to 5
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

//A generic type version of the iterator
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

//The difference between the generic and the type associated version is that the generic version could
//also be implemented for strings or any other type. So we'd have multiple imlementations of 
//Iterator for Counter. When a trait has a generic parameter, we can implement that trait for a type
//multiple times, changing the generic type parameters each time. We'd have to provide type annotations
//to indicate which implementation of Iterator that we wanted to use with the Counter next method.

//With associated types, we can't implement a trait on a type multiple times. We can only choose the
//type of Item once in the above example for counter. Here's another example of the advantages of not having to use
//generic types, and instead using associated types. It is based upon a graph structure.

trait GGraph<Node, Edge> {
    // methods would go here
}

trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}

//Let's say we wanted to implement a distance function for the generic graph trait.

// Our function would need to specify the generic type parameters N, E, and G, where G
//  is bound by the trait GGraph that has type N as its Node type and type E as its Edge type.
//  Even though distance doesn’t need to know the types of the edges, we’re forced to declare
//  an E parameter, because we need to to use the GGraph trait and that requires specifying the type for Edge.
fn distance_gg<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    // ...snip...
    0
}

//Here's what the equivalent function for the associated type graph trait.
//Since the edges aren't needed here we don't have to specify the edge types unlike in the above
//generic distance function. We just have to specify the Node type as G::Node. 
fn distance_ag<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
    // ...snip...
    0
}

//If we wanted to use Trait Object then the generic formulation does get smaller.
//It still requires the use of Edge types to be listed though.
//In our examples it is not possible to rewrite distance_ag to use trait objects
//because there would be no way to refer to the associated types in AGraph.
fn distance_tog<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    // ...snip...
    0
}

//It is possible in some cases to use trait objects of traits that have associated types when we don't
//need to use the associated types in the function arguments. We do have to specify the concrete types for the
//associated types though. If we don't provide this constraint Rust can't figure out which impl to match this trait
//object to.
fn traverse(graph: &AGraph<Node=usize, Edge=(usize, usize)>) {
    // ...snip...
}

use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
//Rust does not allow you to create your own operators or overload arbitray operators, but the operations and corresponding
//traits listed in std::ops can be overloaded by implementing the traits associated with the operator. The following
//shows how to overload the + operator for a point structure.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Here's the trait for the + operator.
//If we don't specify the type for RHS it will defualt to be the same as RHS.
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

//From the book:
// Default type parameters are used in two main ways:

    // To extend a type without breaking existing code.
    // To allow customization in a way most users don’t want.

//From the book:
// The Add trait is an example of the second purpose: most of the time, you’re adding two like
//  types together. Using a default type parameter in the Add trait definition makes it easier
//  to implement the trait since you don’t have to specify the extra parameter most of the time.
//  In other words, we’ve removed a little bit of implementation boilerplate.

//From the book:
// The first purpose is similar, but in reverse: since existing implementations of a trait won’t have
//  specified a type parameter, if we want to add a type parameter to an existing trait, giving it a
//  default will let us extend the functionality of the trait without breaking the existing implementation code.

//Rust does not prevent a trait from having a method with the same name as another trait's method. It also won't
//prevent us from implementing multiple traits with the same name on the same object.

//The below shows us multiple implementations of the fly method onto the human struct.

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


//Here we're going to show what happens when two types in the same scope implement the same trait without the use of
//self. Rust can't figure out this normally unless we give the full specifications when called.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


use std::fmt;
//The below is going to be an example of the use of supertraits, or in other words
//the case where want to have a trait rely on another trait also being implemented.
//So, we can then make use of the other traits functionality in our own trait. This
//other trait is the supertrait we're implementing.

//In the below we need to make sure that self implements Display. As the book puts it,
//it's like adding a trait bound to a trait. 
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//If we want to use OutlinePrint for our point structure we need to make sure we implement the Display trait
//first or else OutlinePrint will fail to compile.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

//Earlier when going over traits we talked about the orphan rule which said that we're allowed
//to implement a trait on a type as long as either the trait or the type is local to our crate.
//We can apparently get around this by is the use of the newtype pattern which involves creating
//a new type using a tuple struct with one field around the type we want to implement the
//trait for. There's no runtime penalty for using newtypes they are elided at compile time.

//The below is an example of a newtype where we implement Display on Wrapper which uses
//a Vec of strings.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //the self.0 access the inner Vec inside the tuple. However by using
        //a new type, we don't have access to all of the methods available in
        //the original type. If we wanted to access all of the methods then
        //we could use the Deref trait on the wrapper to access those types.
        //If we only want a few of the methods then we would need to reimplement
        //all of them for our new type.
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("Hello, world!");
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 });

    //If we want to use the different fly functions we need to
    //specify the trait name before the method call so Rust knows which ones to
    //actually use. Since these trait methods were implemented using the self parameter
    //Rust can figure out which implementation of a trait to use based on the type of
    //self.
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    //This example will work
    println!("A baby dog is called a {}", Dog::baby_name());
    //This example won't because Rust can't figure out which one is the
    //appropriate one to call since we have two different types that
    //implement the same trait.
    // println!("A baby dog is called a {}", Animal::baby_name());
    //This example will work because we used the fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    //In general the fully qualified syntax is as follows:
    //Associated functions do not require a receiver we would only have the
    //list of the other arguments.
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

}
