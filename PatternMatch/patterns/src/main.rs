//Patterns can pop up in a number of different places in Rust

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//Function parameters can also be considered patterns as well.
//We can see that the parameters are a pattern of a tuple with 2 int32 types.
//If the expression does not meet this requirement then the code won't 
//compile.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

//This is an example that shows us that the first variable will match to any expression.
//However, it won't bind to a specific name.
//This can be useful for example when you want to implement a trait and you need a specific
//type signature. However, the actual function body of your implementation does not
//require the additional parameter.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    //The match expression is one example of a pattern. In a match expression it is required
    //that all possibilities are considered. The catch all _ pattern can be used to lump
    //together all of the conditions that you might not care so much about. It is often used
    //in the last match arm.
    

    //If let expressions are mainly used as a shorter equivalent version of match. However, it is
    //possible to mix and max patterns together in a way that is not possible in a match statement.
    //The below is an example of such.

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    //It should be noted that one failing in this use case is that the compiler does not
    //check to make sure things are exhaustive. So, it is possible for logic errors to
    //exist that the compiler did bring up to us.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //Similar in construction to the if let condition is the while let. It will continue to run as long as
    //a condition is true. The below example will keep running until a None is returned.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    let v = vec![1, 2, 3];
    //For loops also use pattern matching.
    //The enumerate method returns a tuple of the index and value.
    //This value is then matched up with the loop index and value variables
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    //The below is also an example of a pattern!
    //let PATTERN = EXPRESSION;
    let x = 5;
    //Here's a good example where the compiler matches a pattern of a tuple with 3 variables
    //with an expression of a tuple with 3 variables.
    let (x, y, z) = (1, 2, 3);
    //If we changed it to the below the compiler would get mad at us and say that it expected
    //a tuple with 3 values and only found one with two for the pattern.
    // let (x, y) = (1, 2, 3);
    //However if we want to just skip a value we can do the following
    let (x, y, _) = (1, 2, 3);

    let point = (3, 5);
    //When we build this we'll see that our inputted expression does run because it meets
    //the requirements of the function parameter pattern.
    print_coordinates(&point);
    //Just like function patterns closures parameters can also be considered patterns.

    //So patterns come in 2 forms refutable and irrefutable.
    //irrefutable patterns are those that will match to any possible value.
    //For example x in the below example is an irrefutable pattern.
    let x = 5;
    //A refutable example is one that may fail to match so for example the
    //below is an example of one.
    let a_value: Option<&i32> = None;
    //So if a_value is not Some() and instead none then it won't run
    if let Some(x) = a_value{
        println!("This is a refutable example");
    }

    //let statements, function parameters, and for loops can only accept
    //irrefutable patterns since the program can't continue doing anything
    //meaningful with values that don't match.

    //if let and while let are examples of refutable patterns. They are
    //meant to be handle the case where a pattern might fail.

    //The below will fail. We'll see that it says the pattern none is not covered.
    // let Some(x) = a_value;
    //We can fix the above by using an if let where the code will skip this part if
    //the pattern is matched
    if let Some(x) = a_value {
        println!("{}", x);
    }

    //We can't use an irrefutable pattern in a place where it expects a refutable
    //pattern. The example below will error on us.

    // if let x = 5 {
    //     println!("{}", x);
    // };

    //The below is an example of matching against literals.
    //This is useful if you want to match against concrete values.
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    //In the below example we are matching against x and inside we
    //are shadowing variables. So for example, y will be the value inside the Some of x
    //outside of this match scope if it isn't a none type.
    //Shadowed variables can be seen to not overwrite the original values.
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    //The below shows an example of where we can match against multiple
    //variables.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    //This is an example where we can match to a range of values by
    //using the ... syntax. Ranges are only allowed for numeric or char values.
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }

    //The below is an example of a char range matching
    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //The below is an example of destructuring structs
    let p = Point { x: 0, y: 7 };
    //The code creates variables that match the values of the
    //x and y fields of the p variable.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    //While we can name the variables different things than the struct field names,
    //we could just as easily name them the same thing as the struct field names.
    //If we do that we actually don't need to put the field names in there along with the
    //variables.
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    let p = Point { x: 0, y: 7 };
    //We can also destructuring structs into a match statment.
    //However, we can see that if both x and y are at the origin that only the first
    //arm will run which is guess what another condition that we need to consider.
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    //We can also destructuring enums in a match expression. When doing this though, the patterns
    //that we destructuring for an enum should match the way the data is stored in the enum.
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        //If an enum has a struct like term to it then we can destructuring that just like we would
        //a struct.
        Message::Move { x: x, y: y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        //In the case of destructuring a tuple we should follow the same format we've used before in
        //destructuring tuples.
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    //When the values we're matching to our pattern contain a reference we need to destructure the reference from the value.
    //We can do this by using the & pattern.

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    //The below is an example of this.
    //We want the variables holding the value that the reference points to rather
    //than getting a variable that holds a reference.

    //This is useful in closures where we have iterators that iterate over references, but we
    //want to use the values in the reference in the closure rather than the references.
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    //Here is an example of how to destructure complex data types that are made up of nested structures.
    //We can see that we are destructuring it such that we have individual variables for each component of the
    //the complex data type.
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    //This example shows us a function parameter that ignores the first parameter.
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    //The below is a good example for where we really don't care certain conditions of a value
    //we really want to just test for certain conditions. For example in the below case,
    //we want to see if a setting is already set as Some value. If it is then we aren't
    //going to overwrite it with a new value. However, if we find this isn't the case then
    //we can set the setting to a new value.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    //We can also use the underscore in cases where we want to ignore certain values in a tuple.
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("Hello!"));
    //We can also use the _varname in cases where we are just starting prototyping and don't want to see unused variable warnings in our code.
    //The big thing to note in these cases is that _varname will still have the data binded to it. 
    //The below case will fail at the println!("{:?}", s); because _s has the new binding of the data.
    // if let Some(_s) = s {
    //If we change the if let to below then the code will compile since the data in s wasn't moved to _s.
    if let Some(_) = s{
        println!("found a string");
    }

    println!("{:?}", s);

    //We can use the .. to say that we want to ignore everything but those listed.
    //However, it can't be ambiguous what values you want listed. For example if we did something like:
    //let (.., middle, ..) = (1,2,3,4,5);
    //The exact value for what middle is ambiguous so the compiler will error on us.
    //The below is fine along with
    let (x, ..) = (1, 2, 3);
    let (first, .., last) = (1,2,3,4,5);

    let robot_name = Some(String::from("Bors"));
    //In pattern matching if we just want the code to have a reference of the data
    //we need to use the ref keyword since the & says to match the existing reference in the value.
    //Therefore the keyword ref needs to be used instead.
    //If we want a mutable reference of the data then we need to use ref mut instead of the usual
    //& mut.
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);


    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        //We need to use the * dereference operator here because name is a mutable reference
        //and we want to mutate the actual value of it.
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    let num = Some(4);

    match num {
        //The below is an example of a match guard which is an additional if condition
        //specified after the match arm.
        //We can also string them together like
        // 4 | 5 | 6 if y => println!("yes"),
        //where this is equivalent to (4 | 5| 6) if y
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    enum Message1 {
        Hello { id: i32 },
    }

let msg = Message1::Hello { id: 5 };

match msg {
    //The @ operator lets us create a variable that holds a value at the same time
    //that we're testing the value to see if matches the pattern.
    Message1::Hello { id: id_variable @ 3...7 } => {
        //This then let's us specifically get out what the value was in that range.
        println!("Found an id in range: {}", id_variable)
    },
    //In this example it doesn't know what value it was in the range only that
    //it met the requirement.
    Message1::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    //Finally we have just a generic case where it wasn't in any of the above
    //ranges.
    Message1::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}

} 