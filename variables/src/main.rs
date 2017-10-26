#![allow(unused_variables)]
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Number is: {}", guess);

    let x: f64 = 2.0; //f64 is double precision
    let y: f32 = 3.0;//f32 is single precision

    let sum: i32 = 5 + 10;
    let diff: f32 = 95.5 - 4.3;
    let product: i32 = 4 * 30;
    let quotient: f32 = 56.7 / 32.2;
    let remainder: i32 = 43 % 5;

    println!("s: {}, d: {}, p: {}, q: {}, r:{}", sum, diff, product, quotient, remainder);

    let t = true;
    let f: bool = false; //boolean types

   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, cat: {}", c, z, heart_eyed_cat);

    //tuple section
    let tup: (i32, f64, u8) = (500, 6.4, 1); //directly giving it the types

    let tup = (500, 6.4, 1); //letting the compiler infer the types

    let (x, y, z) = tup; //deconstructing the tuple kinda similar to python

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; //we can access stuff directly from a tuple using
                            //the index which is 0 based

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5]; //Example of an array. Arrays in rust can not change size.
    //Arrays are put on the stack instead of the heap.

    let first = a[0];
    let second = a[1];

    //Below 3 commented lines kill the program
    //let index = 10;

    //let element = a[index];

    //println!("The value of the element is: {}", element);

    //Now we are going to test functions yeah!

    another_function(5, 6);

    //Rust statements are instructions that don't return a value. Kinda sounds similar to void functions in C.

    //Bad code: let x = (let y = 6);

    //Rust expressions are instructions that do return a value.
    //They also do not end in semicolons

    let x = 5;

    let y = {
        let x = 3; //This only has a lifetime inside these brackets
        x + 1 //This is an expression instead of a statement due to the lack of a semicolon
        //Therefore it should return 4
    };

    another_function(x, y);

    //Now onto control flow stuff

    let number = 3;

    //control flow as you would expect should only take in booleans
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //control flow when defining variables

    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };

    //Below is code that won't compile because the types
    //in the if else statement don't contain the same type
    //This error is caught by the compiler
    //If the variable can't figure out what type a variable
    //is at compile time it will error out
    //let number = if condition {
    //    5
    //} else {
    //    "six"
   // };

    println!("The value of the number is: {}", number);

    //Loops!

    //loop keyword will continue to loop until it runs across a break statement
    //    loop {
    //    println!("again!");
    //}

    //while loop - it requires a conditional in order to run or else it won't compile

    let mut num = 3;
    //This code does run because we gave it a conditional
    //So we can still get infinite loops if we don't
    //ever meet our condition
    while num != 0{
        println!("Test");
        num = num - 1;
    }

    //Now we're going to be looking at the for loop
    //It looks like it is similar to how python's for loop works

    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is: {}", element);
    }

    //A for loop where we use a range of values
    //and in this case reverse that range
    //In rust a Range appears to be something like
    //(#1..#2) so slightly different than pythons range(#2)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is {}", y);
}
