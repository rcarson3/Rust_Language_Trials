fn main() {

    //We currently use owned types so our data is valid for as long as the entire
    //struct is valid.
    //Apparently we can use what are called lifetimes to get around this.
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //We can use the dot notation to access the fields in a struct
    println!("{}", user1.username);

    user1.username = String::from("anotherusername123");

    println!("{}", user1.username);

    //We can create new instances from old instances using several of the
    //old instances values

    //We can even make it exactly the same using the .. syntax
    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("ausername123"),
        ..user1
    };

    println!("{}", user2.username);

    let black = Color(0, 0, 0);

    //Now running an example with structs
    //It's just a simple example calculating the area of a rectangle
    //We also could have just as easily used a tuple for this example
    let rect1 = Rectangle{ length: 50, width: 30};

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1));
    
    //If we want to print a struct we need to do a bit more to get println
    //macro to be happy.
    //We have to use the #[derive(Debug)] to add this trait to our rectangle
    //Now we can use println with the debug formatting option {:?}
    //We can also make it look better by using {:#?}

    //Traits will be covered in more detail the same time we go over lifetimes
    //We'll then be able to make our own as well as use several of the builtin ones

    println!("rect1 is {:#?}", rect1);

    //We will now cover the Methods
    //Methods are essentially functions defined in a struct, enum, or trait object
    //The nice thing about them is we implement them all into one section of
    //code therefore we can easily find all the functions/methods directly related
    //to our struct

    //Here's it being used in practice.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area());

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //An example of using an associated function to initialize a new struct
    //This function is namespaced by the struct: the :: syntax is used for 
    //both associated functions and namespaces created by modules
    let sq = Rectangle::square(3);
}

#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32,
}

//Taken from the rust book.
//The main benefit of using methods instead of functions, in addition to 
//using method syntax and not having to repeat the type of self in every 
//method’s signature, is for organization. We’ve put all the things we can do 
//with an instance of a type in one impl block rather than making future users of 
//our code search for capabilities of Rectangle in various places in the library 
//we provide.

//We can also split this impl block into multiple impl blocks and this would
//be perfectly valid. Apparently this will be more useful when we get to looking
//at generic types and traits.
impl Rectangle{
    //Methods can take ownership of self
    //Therefore we usually borrow self so we don't lose our
    //original self
    //We can also borrow self as an immutable or mutable object
    fn area(&self) -> u32{
        self.length * self.width
    }
    //An example of a method with more parameters
    //We can add multiple parameters to a method after the
    //self parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    //We can also have something called associated functions
    //Associated functions are functions defined in impl that don't
    //take the self parameter. They are often used to return new
    //instances of a struct

    fn square(size: u32) ->Rectangle{
        Rectangle {length: size, width: size}
    }

}

//Structs are similar to tuples in rust except that we name the variables
//They allow us to create custom types that keep the associated data
//together. Then methods of stucts allow us to specify the behavior
//that instances of our structs have. Finally associated functions let
//us have name space functionallity that is particular to our struct but
//we don't have to an actual instance of the struct created.
//This could be quite useful in numerics but I need to think over how
//I might use them first.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Tuple structs are like tuples but have type information in them.
//This can be quite useful if we know what data should makeup a tuple
//and we want it to error if it doesn't receive that datatype 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User{
    //The below could be a pain if we have several fields to fill out
    //A shorthand does exist to initialize structs
    // User{
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(dims: &Rectangle) -> u32{
    dims.length * dims.width
}


