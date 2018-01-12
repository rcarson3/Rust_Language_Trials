#![allow(unused_variables)]
fn main() {
    //Here we are creating an empty i32 vector.
    let v: Vec<i32> = Vec::new();
    //Vec<T> type provided by the standard library can hold any type.
    //However, we told Rust that we want a vec that only hold i32 types.

// From the rust book
    // In more realistic code, Rust can often infer the type of value we want 
    // to store once we insert values, so you rarely need to do this type 
    // annotation. It’s more common to create a Vec<T> that has initial values,
    // and Rust provides the vec! macro for convenience. The macro will create
    // a new vector that holds the values we give it. Listing 8-2 creates a 
    // new Vec<i32> that holds the values 1, 2, and 3:

    let v = vec![1, 2, 3];

    //Now we are going to go over how to update vectors.

    let mut v = Vec::new();
    //The push command allows us to add more values to a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    //Rust infers that all of the data is i32 so we don't need the i32 notation
    // v.push(8.0);
    //If we try the above it failes because the compiler was expecting an integer when
    //it received a float instead.


    println!("{:?}", v);

    {
        let v2 = vec![1, 2, 3, 4];
    } //v2 goes out of scope here and so the memory is freed here
    //so when a vector goes out of scope all of its values will be freed
    //apparently it gets more complicated when we start referencing the internal
    //values of the vec

    let v = vec![1, 2, 3, 4, 5];
    //Once again Rust is zero based
    //The first method of &[] gives us a refernce to the 3rd element
    let third: &i32 = &v[2];
    //The other method is by using the get method which returns an Option<&T> so I believe a reference
    //to the element but as an option
    let third: Option<&i32> = v.get(2);

    //The reason for the two options is based on what happens if you try to access an element outside of the
    //vector bounds.
    let v = vec![1, 2, 3, 4, 5];
    //The first method causes a panic by the compiler
    // let does_not_exist = &v[100];
    //The second method will either return Some(&element) or None
    //This allows you the option to either more peacefully kill the program
    //or it allows you to give the user another chance to input a correct
    //index.
    let does_not_exist = v.get(100);

    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];
    //This will fail because the first element is currently being referenced.
    //The reason behind this is that if there is not enough memory for that element
    //to be added to the end of the vector, then Rust will copy the vector to a new
    //memory location with enough memory. Then it will deallocate the old memory space.
    //Therefore, first would now be pointing to an invalid memory location which is not
    //kosher.
    // v.push(6);

    //We can iterate over a vec by doing the below.
    //It should be noted that we are just taking a reference of the vec
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //We can also change the values of the vector by doing the following:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //here * is the dereference operator which allows us to get the value of i
        *i += 50;
    }
    //We can now see that v has been incremented.
    for i in &v {
        println!("{}", i);
    }


    //Now what happens if we want to store multiple different data types inside a vec?
    //We can use the Enum type which therefore allows us to create a vec of that 
    //enum, but inside we can store different values inside of it that correspond to
    //the types of the variants of the enum.


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

// From the rust book
    // The reason Rust needs to know what types will be in the vector at compile time is so
    //  it knows exactly how much memory on the heap will be needed to store each element. 
    // A secondary advantage is that we can be explicit about what types are allowed in this 
    //vector. If Rust allowed a vector to hold any type, there would be a chance that one or 
    //more of the types would cause errors with the operations performed on the elements of 
    //the vector. Using an enum plus a match expression means that Rust will ensure at compile
    // time that we always handle every possible case, as discussed in Chapter 6.

    // If you don’t know the exhaustive set of types the program will get at runtime to store 
    // in a vector when you’re writing a program, the enum technique won’t work. Instead, you 
    // can use a trait object, which we’ll cover in Chapter 17.


    // Now that we’ve discussed some of the most common ways to use vectors, be sure to review 
    // the API documentation for all the many useful methods defined on Vec<T> by the standard
    //  library. For example, in addition to push, a pop method removes and returns the last 
    // element. Let’s move on to the next collection type: String!


}
