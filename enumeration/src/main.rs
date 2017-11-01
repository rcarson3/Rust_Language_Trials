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

    //The enum Option type from the stdlib is a very common type due to it
    //being able to give the option of something existing or not at all. This
    //type is even included in the prelude so we don't need to import it.
    //It's variants, Some and None, also don't need to be explicitly imported.
    //They are still variants of type Option<T>. The <T> syntax in Rust represents
    //a generic type parameter. This just means that Option::Some can hold one
    //piece of any data type.

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    //If we use None instead of Some the compiler needs to be told what data type
    //Option is.

    //So why Option<T> instead of just a null? Well here's an example that doesn't
    //compile:
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    //It doesn't compile because Rust doesn't know how to add an Option<i8> to an i8.
    //We need to convert our Option<T> to a T and Rust has several builtin methods that
    //can be found in the documentation of Option. However, one option that could allow this
    //is the match expression which is a control flow construct that runs some code when
    //we have a Some(T) value where T can be accessed and other code if we have a None value.

    //We are now going to go over the Match control flow operator. It kinda reminds me of like
    //switch statement. However, it sounds like it might be slightly more powerful.

    // enum Coin {
    // Penny,
    // Nickel,
    // Dime,
    // Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u32 {
    //     //The match keyword is followed by an expression
    //     //This expression can be any type
    //     match coin {
    //         //These following parts are the match arms. They have two parts a pattern and some
    //         //code. The => operator seperates the pattern and the code.
    //         //Each arm is seperated by a comma. The match expression checks the patterns in order.
    //         //If a pattern matches it runs the code. If it doesn't goes down the list. The code 
    //         //associatted with the pattern is an expression. The resulting value of the expression
    //         //is what is returned for the whole match expression.
    //         //If we have long match arm code we can put the code in curly braces.
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            //In the match expression we add the state variable which would be of
            //the data type that the Quarter contains. We can then use this variable
            //in our match arm code.
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let price = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("{}", price);

    //Now here's a section for the matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        //This match works the same as the above code.
        //If we forget a case when creating our match the compiler will throw an error.
        //This is due to the Rust compiler checking to make sure that all cases are covered and
        //that none are left out.
        match x {
            None => None,
            //i binds to the value contained in Some so in our
            //example it takes the value 5. We then add 1 to this value.
            //Then we return a new Some value.
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        //This is a placeholder value saying that any case not covered by the above should fall
        //under this case. It's very similar to the default case in a switch statement. The () given
        //here is just a unit value so nothing happens in this case.
        _ => (),
    }

    //If we are only interested in just one case Rust provides the if let expression.

    //We can think of if let to be syntax sugar for a match statement with a single arm and
    //a placeholder that evaluates to nothing.
    //So the following are the same...

    // let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    //The above is the same as the following:
    let some_u8_value = Some(0u8);
    //The if let takes a pattern followed by an expression seperated by an =
    if let Some(3) = some_u8_value {
        println!("three");
    }

    //We can also do an if let else statement and this now allows us to execute
    //some expression if we did not match our pattern. 

    //From the book: Summary
    //We’ve now covered how to use enums to create custom types that can be one of a set of enumerated 
    //values. We’ve shown how the standard library’s Option<T> type helps you use the type system to 
    //prevent errors. When enum values have data inside them, you can use match or if let to extract and 
    //use those values, depending on how many cases you need to handle.
    //
    //Your Rust programs can now express concepts in your domain using structs and enums. Creating custom 
    //types to use in your API ensures type safety: the compiler will make certain your functions only get
    //values of the type each function expects.


}