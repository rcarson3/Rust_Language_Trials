fn main() {
    //Enums in Rust are most similar to algebraic types in functional languages
    //such as OCaml or Haskell

    // //Example of an enum pretty similar to what we might do in C
    // enum IpAddrKind{
    //     //The below are the known variants of the enum
    //     V4,
    //     V6,
    // }

    //We could use them in a struct to create an internet data type for example as
    //follows

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }


    // let home = IpAddr {
    // kind: IpAddrKind::V4,
    // address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // //We can also represent this struct as an enum!
    // //We are going to redefine IpAddr as the following
    // enum IpAddr {
    // V4(String),
    // V6(String),
    // }
    // //We can attach data to each variant of the enum directly
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    //All of the above had to be commented out or else we would get compiler
    //errors about IpAddr already being defined
    //We can only define a type once in the namespace of a block of code

    //We can also change this up a bit and actually make this a bit cooler than
    //structs by assigning unique data types to each variant.
    //The type of data stored in an enum can be from the sounds of it pretty much
    //anything from strings, numeric types, structs, or even enums.

    enum IpAddr{
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    //Here we have a more complex enum where each variant stores not only
    //different data types but also the quantities of the data.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    //It's shown in the rust book that we could make each one of these variants
    //their own struct type. However, if we did this then we would be able to have
    //them interact with each other.

    //We can also define Methods for enums as seen below:
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //The body of the method would use self to get the value that we called the 
    //method on. In this example, we’ve created a variable m that has the value 
    //Message::Write("hello"), and that is what self will be in the body of the 
    //call method when m.call() runs.

    

}