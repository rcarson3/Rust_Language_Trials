//Lifetime annotations don't change how long a reference lives. It just relates 
//the lifetimes of multiple references to each other.

//The life time syntax requires the name to be followed by an apostrophe '.
//The name is usually a single lower case character.
//The annotation goes after the & of a reference, and a space seperates it the type
//of the var.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


//This fails because the compiler can't tell what the lifetime is of that borrowed value
//since the lifetime of the var is from either x or y.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//The new version of the above function with lifetimes now. We can see that the
//lifetime needs to go inside the angle brackets of the function just like generic
//types.
//We are telling the compiler that these variables have some lifetime that will
//live at least as long as a'
//When concrete references are passed to the compiler: a' will get the concrete lifetime
//of the shorter lived variable x and y. We therefore are guaranteed that our return
//value will live at least as long as the shorter lived variable.
//
//We also don't need to apply the lifetime annotation to a variable if we know
//it will never be used in the return value. Example given below:
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//Lifetime Elision rules:
// 
//If the compiler can work through these 3 rules without problems then we don't
//need to add lifetime annotations.
// 
// First, some definitions: Lifetimes on function or method parameters are called
//  input lifetimes, and lifetimes on return values are called output lifetimes.
// 
// Now, on to the rules that the compiler uses to figure out what lifetimes references
//  have when there aren’t explicit annotations. The first rule applies to input 
// lifetimes, and the second two rules apply to output lifetimes. If the compiler 
// gets to the end of the three rules and there are still references that it can’t 
// figure out lifetimes for, the compiler will stop with an error.

    // Each parameter that is a reference gets its own lifetime parameter. In other 
    // words, a function with one parameter gets one lifetime parameter: 
    // fn foo<'a>(x: &'a i32), 
    // a function with two arguments gets two separate lifetime parameters: 
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.

    // If there is exactly one input lifetime parameter, that lifetime is assigned 
    // to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    // If there are multiple input lifetime parameters, but one of them is &self
    //  or &mut self because this is a method, then the lifetime of self is
    //  assigned to all output lifetime parameters. This makes writing methods
    //  much nicer.

//We can also allow structs to hold references. However, if we do this then we need
//to impose lifetime annotations into the struct.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//We also have to apply lifetime annotations to implementations that are for a struct
//that contains lifetimes
impl<'a> ImportantExcerpt<'a> {
    //here we don't have to apply a lifetime because of rule one and two
    fn level(&self) -> i32 {
        3
    }
}
//Here's an example where rule 3 applies
impl<'a> ImportantExcerpt<'a> {
    //From rule 1 self and str are both assigned their own lifetime.
    //Then we have to move to rule 3 where we can see because we have a &self
    //then the lifetime of the output can be just assigned to be the same as the structs
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//There also exists a static lifetime called 'static which says that a variable
//will live for the entire lifetime of the program.
//All string literals are assigned a lifetime of 'static because the text is directly
//stored in the binary of the program which is always available.

//You need to really think before adding this to a variable if it actually is one that
//will live the entire lifetime and isn't just one that will lead to a dangling reference


use std::fmt::Display;

//Here's an example with generic types, trait bounds, and lifetimes all put into one
//lifetimes are a type of generics which is why it goes in the brackets with the 
//generic type var.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //Example where the we declare results along with the
    //shortest lifetime.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    //Example where we declare result outside of the shortest lifetime and try to use
    //it outside of this.
    //It will fail because the shortest lifetime does not live for long enough.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    //     //If we move the println inside this block the code still fails.
    //     println!("The longest string is {}", result);
    // }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    //Here we can see that part will be a reference to a string
    let i = ImportantExcerpt { part: first_sentence };

}