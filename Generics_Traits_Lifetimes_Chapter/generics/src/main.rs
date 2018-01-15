
//Generics allows us to create definitions in things like functions signatures
//or structures that can represent many different data types.

//The below shows us two functions that are exactly the same except for the data types.
// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

//The generic version of the code can be given as:
//<T> this is the name of the type parameter. It can be anything but
//typically Generic type parameters are short by convention also Rust's type naming
//convention is CamelCase. Type name declarations go in the <> between the fcn name
//and the parameter list.+ Copy
fn largest<T: PartialOrd>(list: & [T]) ->  & T{
    let mut largest = &list[0];

    for ref item in list.iter() {
        //This statement will cause the code to fail because not all types know how to
        //be ordered.
        //We've fixed this issue by requiring the types to be trait bounded by the above
        //This is now implemented for generic types that don't need to have any copy on them
        //Sweet! Also this appears how to compare items for a reference to the original
        //One thing though is we later on can't modify the reference that is returned.
        if  item > &largest {
            largest = *item;
        }
    }

    largest   
}

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

//We'll return to the above later on.
//We define the struct generic type parameter just like we did with functions.
// struct Point<T>{
//     x: T,
//     y: T,
// }

//We have to declare <T> right after impl in order to use T in the type Point<T>
//This is needed so that Rust knows that T in Point is a generic type and not
//some defined type.
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

//We can also define implementations for concrete types of a generic type.
//If we do this then only that concrete type will have access to the method.
//The generic implementation methods will still be available to the types that
//have the specific methods.
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

//Here we define Point to be capable of holding 2 types
//We can use as many generic types as we want in a definition.
//However, if we start getting too many where the code becomes hard to read
//then you might want to think about restructuring the code.
struct Point<T, U> {
    x: T,
    y: U,
}

//Here we give an example for where we the methods in the implementation can have
//different types than those defined within for that structure
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//One more thing about Generics when the compiler compiles all of this it generates
//only the types it needs for the generics. It doesn't create anything extra. Therefore,
//we achieve the same performance as if we'd created seperate functions for each one of
//the types that we'd created.
fn main(){
    // let integer = Point {x: 5, y: 10};
    // let float = Point { x: 1.0, y: 4.0};
    //The below won't work because we only defined Point to be only of type T.
    // let wont_work = Point { x: 1, y: 5.0};

    //Example of multiple data types generic structs
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // let p = Point { x: 5.0, y: 10.0 };
    //Here we can see that the generic implemenations are available to Point of type float
    // println!("p.x = {}, distance from origin {}", p.x(), p.distance_from_origin());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("Number list is still {:?}", number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    //This example shows how we can have new types returned to us that aren't either
    //the same as the first two given.
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

//We can also use generics in Enums and we've already come across this in terms of
//Option and Result as seen below:

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


