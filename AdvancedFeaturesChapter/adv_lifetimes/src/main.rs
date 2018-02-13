
// Rust will let you elide lifetimes, but every reference has a lifetime. There are three
//  advanced features of lifetimes that we haven’t covered though: lifetime subtyping, 
// lifetime bounds, and trait object lifetimes.

//In order to use any of the below we need to first provide lifetimes to Context and Parser
//Our first attempt failed because we told the compiler that Context and Parser have the same
//lifetime. This may not be the case as seen in our parse_context function. So, what if we give them
//both different lifetimes...
//Once again this fails istead we need use something called lifetime subtyping to tell the compiler
//that the reference to the string in Context lives at least as long as parser context lifetime.
struct Context<'s>(&'s str);
//We'll just say that Parser and Context have the same lifetime.
//If we build it we'll see that it now compiles.
//We found that the above didn't work and after a bit of trial and error we found that we needed to use
//lifetime subtyping. 'a: 'b is lifetime subtyping and it says that lifetime 'a will live at least as long as
//lifetime 'b. We won't need to use this feature often but it can come up when you need to refer to something
//you have a reference to like our given example.
struct Parser<'a, 's: 'a> {
    //Out parser will need to borrow a reference to our string
    context: &'a Context<'s>,
}
//However let's say we add the following to our code. We should see that
//we no longer are able to compile our code because the compiler does not
//believe that context that is going to be used in Parser will live long enough for this function.
//So we need to change the lifetimes of the structs a bit more again.
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

impl<'a, 's> Parser<'a, 's> {
    //The parser will need to parse the str and then pass the results
    //as either a success or a failure.
    //This code won't compile because we left out our lifetime annontations on
    //str and context: &Context
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

//We can add lifetime parameters as constraints on generic types, which are called called lifetime bounds.
//Take the following example which is a wrapper over a reference. This is an example of the Ref type we've used earlier.
//If we try and compile this as is the compiler will error on us and tell us that type T might not live long enough.
//It will also suggest how we can fix this with lifetime bounds.
//struct Ref<'a, T>(&'a T);
//We can fix this by doing the below. What this accomplishes is that it says T can be any type and if there are any references
//in T they must live at least as long as 'a.
struct Ref<'a, T: 'a>(&'a T);
//We could also have fixed the above by telling it that we only want type T to have references with static lifetimes or no references at
//all. The compiler will see those two as essentially the same thing, since static lifetimes mean the reference is valid for the entire
//lifetime of the program.
struct StaticRef<T: 'static>(&'static T);


trait Foo { }
//From the book.
// This code compiles without any errors, even though we haven’t
//  said anything about the lifetimes involved in obj. This works 
// because there are rules having to do with lifetimes and trait objects:

    // The default lifetime of a trait object is 'static.
    // If we have &'a X or &'a mut X, then the default is 'a.
    // If we have a single T: 'a clause, then the default is 'a.
    // If we have multiple T: 'a-like clauses, then there is no default; we must be explicit.

// When we must be explicit, we can add a lifetime bound on a trait object like Box<Foo> with the syntax Box<Foo + 'a>
//  or Box<Foo + 'static>, depending on what’s needed. Just as with the other bounds, this means that any implementor
//  of the Foo trait that has any references inside must have the lifetime specified in the trait object bounds as those references.
struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

fn main() {
    println!("Hello, world!");

    let num = 5;

    let obj = Box::new(Bar { x: &num }) as Box<Foo>;
}
