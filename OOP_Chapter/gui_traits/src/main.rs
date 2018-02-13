//Also didn't really go into this too much but we can implement state patterns into Rust which means
//a values behavior changes based upon what an internal state of the system is set to.
//I didn't really go over a lot the examples for that just because at the time it doesn't feel like something
//I'll use as much. It also didn't appear to have the best examples for that section. So, it could
//be more useful for certain applications in numerics.

extern crate gui_traits;
use gui_traits::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
    }
}


use gui_traits::{Screen, Button};
//The nice thing of trait objects is that we never have to worry about whether or not an object contains
//the necessary traits at runtime. If they don't then Rust won't compile the code.

//However, trait objects have some disadvantages to them. The compiler can't create static dispatches of the code
//at compile time. Instead, it'll need to use a dynamic dispatch which involves the use of pointers inside the object,
// so it can figure out at runtime what are the appropriate methods to call using a lookup table. We take a hit here in
// performance. The compiler also can't inline functions so we lose out on additional optimizations that might occur.
//Nonetheless, it does provide us with quite a bit of flexibility in the code that we write which might be worth the
//tradeoffs listed above. In numerical code, we'll probably want to go with generic types a majority of the time for the
//extra performance gains. A few places that we might be able to flexible in this sort of thing might be post-processing
//or the initial reading of the data.

//Further rules on traits. Only object safe traits can be made into trait objects. The main rules for this are the following
//The return type isn't self
//There aren't any generic type parameters.
// From the book
// The Self keyword is an alias for the type we’re implementing traits or methods on. Object safety is required for trait objects
//  because once you have a trait object, you no longer know what the concrete type implementing that trait is. If a trait method
//  returns the concrete Self type, but a trait object forgets the exact type that it is, there’s no way that the method can use
//  the original concrete type that it’s forgotten. Same with generic type parameters that are filled in with concrete type parameters
//  when the trait is used: the concrete types become part of the type that implements the trait. When the type is erased by the use 
// of a trait object, there’s no way to know what types to fill in the generic type parameters with.

//An example of this is the Clone trait. For example, the type String implements the trait clone. So when it is called you get 
//a String type back. 
// The below goes more over object safety.
//https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md


fn main() {
    //Here we can see that we've built a screen instance that makes use of our trait objects
    //It contains both a SelectBox and Button type in it.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
    //This will error on us during compilation of the program because String doesn't
    //implement the draw trait.
    // let screen = Screen {
    //     components: vec![
    //         Box::new(String::from("Hi")),
    //     ],
    // };

    // screen.run();
}