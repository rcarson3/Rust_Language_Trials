//We can pass in functions as arguments to other functions as well. This is accomplished by using
//function pointers.

fn add_one(x: i32) -> i32 {
    x + 1
}

//fn is how we denote a function pointer
//fn is also a type rather than a trait.
//We declare the type directly instead of having to use the generic type parameter.
//From the book:
// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce),
//  so we can always pass a function pointer as an argument when calling a function that expects a closure. 

// Prefer to write functions using a generic type and one of the closure traits, so that your functions can
//  accept either functions or closures. An example of a case where you’d only want to accept fn is when 
// interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, 
// but C doesn’t have closures.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// We can return a closure in a function call however it's a little bit trickier since closures
//don't have a concrete type. So we can't use a fn pointer as the return type.
//We can return closures using a Box type. We can do the below because of the Sized trait.
//We can think of this therefore as a sized trait object.

fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    //In the map method we could use a closure to transform a set of
    // vecs to strings or we could use a function
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    //Here we see an example of where in the map method we use a function
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}
