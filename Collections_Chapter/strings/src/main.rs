#![allow(unused_variables)]
fn main() {
    //Rust only has one string type: str. We usually see this in its borrowed form
    //&str. The String type is really a string buffer that is growable and mutable.

    // Rust’s standard library also includes a number of other string types, such as
    //  OsString, OsStr, CString, and CStr. 

    let mut s = String::new();
    //We just made an empty string

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    //The to_string method creates a String from a string literal.

    //We can also use the String::from function to create a String
    // from a string literal

    //Since Strings are UTF-8 encoded all of the below are valid:
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    //A String can grow in size by using the push method, or +, or the format! macro to concatenate
    //String values together.

    //We can append to a String value with push_str and push
    let mut s = String::from("foo");
    //This let's use append a string slice to the end
    s.push_str("bar");

    //push_str takes a str slice so that we can continue to make use of the str
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    //The push method just takes a single character as the parameter.

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Note that s1 has been moved here and can no longer be used
    let s3 = s1 + &s2;
    //The reason behind this is that the signature for add
    //takes self and a reference.
    //Therefore s1 must be consumed in the function

    //If we need to concatenate multiple strings together than things can get much
    //more complicated to see what's going. Therefore this is where the format!
    //macro comes into hand.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //The nice things about this is that it doesn't take ownership of any of the parameters.
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s is now {}", s);

    //Strings don't support typical indexing syntax
    //The reason behind this is due to strings being internally represented as a wrapper over
    //Vec<u8>.

    // let len = String::from("Hola").len(); //This returns 4 as in it takes 4 bytes to represnt the UTF-8
    // let len = String::from("Здравствуйте").len(); //This returns 24 instead of 12 because it takes 2 bytes to represent each char
    //It's because of the above that Strings don't allow the index styling because there is no guarantee that
    //the index given will provide a valid character.

    //Strings can also be viewed as three different things in rust. 1: the byte values,
    //2: the scalar Unicode values that aren't necessarily letters, 3: grapheme clusters
    //which are what we would normally consider the appropriate letters. This is just one
    //more reason indexing isn't allowed for Strings. The final reason is that in Rust 
    //indexing is expected to always be constant time O(1) but this can't be guaranteed for Strings.

    // We can access strings by slicing up a String
    // let hello = "Здравствуйте";
    // 
    // let s = &hello[0..4]; 
    //However if we end slicing something invalid like "let s = &hello[0..1]" the program panics because
    //The 0 index isn't a valid character. Therefore this should be used with caution.

    //We can use the chars method to perform operations on each one of the individual Unicode scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //We can also use the .bytes() method if we need to perform operations on the raw bytes of the strings
    //The standard library doesn't provide a method to view graphme clusters because that is complex.
    //However, there does exist crates on Cargo.io for that purpose.


    //The final point is that strings are complex. 

}
