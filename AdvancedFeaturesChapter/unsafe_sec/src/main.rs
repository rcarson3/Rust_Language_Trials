//Unsafe code allows us to do a number of things that we can't do with safe code.
//The following are 4 things that we can do in unsafe that aren't possible in safe.

    //1. Dereferencing a raw pointer
    //2. Calling an unsafe function or method
    //3. Accessing or modifying a mutable static variable
    //4. Implementing an unsafe trait

//While calling unsafe code the borrow checker and the other Rust
//safety checks are still called.
//Unsafe code allows us to limit the areas we have to check for memory bugs to just
//those areas when things go wrong. Therefore, it's best practice to keep these unsafe
//areas small, so you have less code to check when things go wrong.

//When creating code with unsafe blocks you should create a safe abstraction over it,
//and then create a safe api.

// We'll first be going over raw pointers. Raw pointers come into two different types
//immutable and mutable types. Immutable means that after it's been dereferrenced it
//can't have a value reassigned to it.

// Raw pointers are different than references and smart pointers in a few ways. Raw pointers:

//     Are allowed to ignore the borrowing rules and have both immutable and a mutable
//        pointer or multiple mutable pointers to the same location
//     Aren’t guaranteed to point to valid memory
//     Are allowed to be null
//     Don’t implement any automatic clean-up

//Here we've got a global variable. These can only be mutated using raw pointers.
//A data race can happen if two variables access the data at the same time.
//From the book:
// static variables are similar to constants: their names are also in SCREAMING_SNAKE_CASE 
// by convention, and we must annotate the variable’s type, which is &'static str in this case.
//  Only references with the 'static lifetime may be stored in a static variable. Because of this,
//  the Rust compiler can figure out the lifetime by itself and we don’t need to annotate it explicitly. 

static HELLO_WORLD: &str = "Hello, world!";

// Another way in which static variables are different from constants is that static variables can be mutable.
//  Both accessing and modifying mutable static variables is unsafe. Listing 19-10 shows how to declare, access,
//  and modify a mutable static variable named COUNTER:

static mut COUNTER: u32 = 0;
// Mutable data that is globally accessible is difficult to manage and ensure that there are no data races, which
//  is why Rust considers mutable static variables to be unsafe. If possible, prefer using the concurrency techniques
//  and threadsafe smart pointers we discussed in Chapter 16 to have the compiler check that data accessed from different threads is done safely.
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;
    //Examples of first an immutable raw pointer and finally
    //a mutable raw pointer.
    //These raw pointers are valid since they were created directly from
    //references that are guaranteed to be valid. However, this is not always
    //the case with raw pointers after this short example will show just such an
    //example.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    //The below assigns a pointer to an arbitary memory address. However, we can't guarantee there will
    //be any data actually there. This is undefinied behavior and should never be done in almost all cases.
    //Embedded devices this might be something possible without it being undefinied but I could say.
    // let address = 0x012345usize;
    // let r = address as *const i32;

    //You can create raw pointers in safe code. However, you can not
    //dereference them to read their data in safe code.
    //You'll need to use unsafe code blocks for those uses. Regular references
    //don't allow immutable and mutable references to exist at the same time. However,
    //we can do that here with raw pointers. It should be noted that because of this we
    //can have data races within our program...
    //
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //The second usage for unsafe blocks is making calls to unsafe functions or methods.
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    //We are going to look at an example for how to safely abstract over unsafe code. 
    //We're going to look at implementing split_at_mut as a function that only works with i32 types
    //instead of the actual generic type implementation in the std lib.
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}

//Here's are example of this code with just using safe rust.
//The below function will all fail on us.
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);
//     //Rust just sees this as taking two mutable reference of the data.
//     //It doesn't understand that we are just creating mutable references of two slices that
//     //don't overlap each other.
//     (&mut slice[..mid],
//      &mut slice[mid..])
// }

use std::slice;
// We weren't able to use safe rust to do what we want so we're going to be using unsafe rust.
//We'll see from this example that we've provided a safe abstraction over everything by wrapping the
//unsafe parts into this function.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //Slices are just pointers to our data. We're going to use
    //len to get it's length and then ptr is going to be a mutable raw pointer
    //to our actual data.
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    //You still need to be careful when using slice::from_raw_parts_mut since it can return a slice of data that is outside
    //the scope of a valid set of memory.
    unsafe {
        //This method does the reverse of as_mut_ptr it takes in a raw ptr and creates
        //a slice of the data that is equal to the length we've provided it.
        (slice::from_raw_parts_mut(ptr, mid),
        //the offset method is also unsafe since it assumes that the point given to it
        //is also a valid memory location.
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

// If you need to make FFI calls to external code then you need to use unsafe blocks
//extern C tells us that we are going to be interfacing with the C ABI which is the most popular
//ABI out there. It should be clear why this is unsafe, since we can't make any claims to the safety of the other language.
//
// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }
//The extern keyword also allows us to expose our Rust codes to foreign languages.
//This example of the extern keyword doesn't require the use of unsafe.
//Also, we have to use the no mangle feature to tell Rust not to mangle the name when this is all compiled.
//We can then uses this function in for example C by calling the shared library and linked from C.
// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

// Finally the last thing we can do with unsafe is implement an unsafe trait.
// If we implement a type that contains something that’s not Send or Sync such as
//  raw pointers, and we want to mark our type as Send or Sync, that requires using unsafe
// unsafe trait Foo {
//     // methods go here
// }

// unsafe impl Foo for i32 {
//     // method implementations go here
// }

