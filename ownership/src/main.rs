fn main() {
    //Rust deals with stack and heaps for memory managment no gc or direct memory management
    //The stack memory is a first in last off type queue
    //Stack data must take up a known and fixed size
    //In rust the heap is used for when we don't know the size of the vector at compile time
    //or if the memory to be allocated is dynamic
    //Heap memory is not really organized data just kinda gets thrown where the os has space for it
    //Therefore, the program has to jump around to get data which can slow things down. 
    //Function local variables get pushed onto the stack and then popped off when
    //it's done

    //A value is assigned an owner which is it's owner. Only one owner can exist at a time
    //When the owner is out of scope the value disappears

    //Examples to go over variable scope

    let s = "hello";

    {
        let s = "hello2";
        println!("s: {}", s);
    }

    println!("Previous s is out of scope but the one defined earlier isn't");
    println!("s: {}", s);

    //Onto the next example which goes over the rules of ownership
    //It's going to be using the String type aka StringBuffers

    let mut s = String::from("hello");

    s.push_str(", world!");// s must be mutable for this to work

    println!("{}", s);

    //Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes 
    //called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar 
    //to you if you’ve used RAII patterns.

    let x = 5;
    let y = x;// y is just a copy of x since they are simple types and have a fixed size

    let s1 = String::from("hello");
    let s2 = s1; // s2 is a copy of the pointer to the data that s1 points to
                // this errors out because s1 does not have a copy trait which meanse
                //we made a shallow copy instead of a deep copy. Rust does not like this
                // if we tried to use s1. If we use s2 we are fine since s1 is invalidated
                //after we assign s2 to s1 values. This operation is called a move.

    // println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // This creates a deep copy of of s1. We can now use s1 in other places with out
                        // it being invalid

    // println!("{}",s1);

    //Info about what things that make a deep copy when you do let x = something; let y = x;

    // Rust has a special annotation called the Copy trait that we can place on types like integers that are 
    // stored on the stack (we’ll talk more about traits in Chapter 10). If a type has the Copy trait, an older 
    // variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the 
    // type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen 
    // when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile time error. 
    // To learn about how to add the Copy annotation to your type, see Appendix C on Derivable Traits.

    // So what types are Copy? You can check the documentation for the given type to be sure, but as a general rule,
    //  any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource 
    //  is Copy. Here are some of the types that are Copy:

    // All the integer types, like u32.
    // The boolean type, bool, with values true and false.
    // All the floating point types, like f64.
    // Tuples, but only if they contain types that are also Copy. (i32, i32) is Copy, but (i32, String) is not.


    let s = String::from("hello");  // s comes into scope.

    //So in rust if we pass a variable into a function it loses it's ownership to the
    //function. Then once the function is over that variable no longer exists
    //because it is now out of scope.
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    let x = 5;                      // x comes into scope.
    //If a variable has the copy trait then only a copy is made to the function and
    //we can still use the variable afterwards even though all the variables in the 
    //function are now out of scope.
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

    //we can give ownership of a variable from a function by having an expression at the end.
    //We could pass in a variable and then take back its ownership by doing this. However, I think this
    //is kinda of a pain. The people at Rust feel the same.

    let s1 = gives_ownership();

    //Rust also let's return variables as tuples so which we can then can deconstruct this when
    //we get the returned values.


    //Now it's time to go over references and borrowing!

    let s1 = String::from("hello");
    //The & creates a reference to a variable. They can be thought of a pointer to the original data.
    //By doing this we do not pass ownership of the variable to the function
    //Therefore when we go out of scope of the function we still have ownership of the variable
    //where the function call was made.
    //References as function parameters is called borrowing. 
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //We can not modify a borrowed variable.
    //change(&s1);

    let mut s1 = String::from("hello");

    //We can fix this by making a mutable reference
    //We also need to make sure that our variable we're passing in is also mutable.
    change(&mut s1);
    println!("{}", s1);

    //You are only allowed one mutable reference to a particular piece of data in a particular scope.
    //This insures that we don't have any aliasing with our references refering to the same data.
    //The benefit of having this restriction is that Rust can prevent data races at compile time.

    //From the rust book
    //Whew! We also cannot have a mutable reference while we have an immutable one. 
    //Users of an immutable reference don’t expect the values to suddenly change out from under them! 
    //However, multiple immutable references are okay because no one who is just reading the data has 
    //the ability to affect anyone else’s reading of the data.

    //let mut s = String::from("Hello");
    //let r1 = &s; //Immutable reference
    //let r2 = &s; //Immutable reference
    //let r3 = &s; //Mutable reference -- big no no

    //The compiler does not dangling pointers/references. It therefore will error out on us.
    // let refernece_to_nothing = dangle();

    //We are now going to go over slices.

    //From the rust book: Another data type that does not have ownership is the slice. 
    //Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

    // let mut s = String::from("hello world");

    // let word = first_word(&s); // word will get the value 5.

    // s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    //The index we got is now completely out of sync with our original string.
    //If we end up having more indices we could get even more out of sync with our data.

    //For strings we can take advantage of a built in feature called string slices.
    //They create a reference to portions of a string.

    let s = String::from("hello world");

    //Slicing is similar to slicing in python where you have a starting index and then
    //the ending value is +1 of the data you actually care about.
    let hello = &s[0..5];
    // let hello = &s[..5]; //Equivalent to the above
    let world = &s[6..11];
    // let world = &s[6..]; //Equivalent to the above
    let len = s.len();
    let slice = &s[0..len];
    // let slice = &s[..]; //Equivalent to the above


    // We now have a straightforward API that’s much harder to mess up, since the compiler will 
    //ensure the references into the String remain valid. Remember the bug in the program in Listing 4-11, 
    //when we got the index to the end of the first word but then cleared the string so our index was invalid? 
    //That code was logically incorrect but didn’t show any immediate errors. The problems would show up later 
    //if we kept trying to use the first word index with an emptied string. Slices make this bug impossible 
    //and let us know we have a problem with our code much sooner. Using the slice version of first_word 
    //will throw a compile time error:

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // Error!

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also 
    // take a mutable reference. Because clear needs to truncate the String, it tries to take a mutable reference, 
    // which fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors 
    // at compile time!

    let s = "Hello, world!";

    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string 
    // literals are immutable; &str is an immutable reference.

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    // This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the 
    // first element and a length. You’ll use this kind of slice for all sorts of other collections. We’ll discuss
    //  these collections in detail when we talk about vectors in Chapter 8

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

//Tell what type the function will return
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
//This function will error on us since we are trying to
//modify a borrowed variable. We will always get an 
//error for this function even if we never call it.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//This fixes the above code by making a mutable reference that we can now modify.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//The below code creates a dangling pointer/reference.
//So when the data goes out of scope at the end of the function
//our reference now points to memory that has been freed.
//The compiler catches this and errors out on us.
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

//This version doesn't create slices of the data so things become out of index with each other
//We are going to rewrite it with a new version
// fn first_word(s: &String) -> usize {
//     //We are converting our string into a byte
//     let bytes = s.as_bytes();
//     //We now iterate through the string using iter.
//     //the enumerate function packages up each part of the
//     //iterator as a tuple with an index and a reference to the value
//     for (i, &item) in bytes.iter().enumerate() {
//         //We check to see if the byte literal of the space is 
//         //equal to our item.
//         //If it is then we return that index.
//         if item == b' ' {
//             return i;
//         }
//     }
//     //If we don't run across a space at all then we return the length of the string.
//     s.len()
// }

//We can change the following to the current function signature
// fn first_word(s: &String) -> &str {
//The new signature now allows us to operate on both Strings and str types
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}