use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//We can get around the multiple evaluations of the closure by
//creating a struct that will hold the closure and it's result.
//This style of doing things is called memoization or lazy evaluation.
struct Cacher<T, B, C>
    where T: Fn(B) -> C,
          B: Copy + Eq + Hash, 
          C: Copy
          
{
    //This tells us that it has calculation field that
    //calculates a u32 value and returns a u32 value.
    calculation: T,
    value: HashMap<B, C>,
    // value: Option<C>,
}

//Closures implement one of these three traits from the std lib.
//The Fn, FnMut, or FnOnce trait.
//Functions also implement all three of the above traits. If
//what we want doesn't require capturing a value from the env we
//could a function rather than a closure.


    // FnOnce consumes the variables it captures from its enclosing scope, 
    //  known as the closure’s environment. In order to consume the captured 
    //  variables, the closure must take ownership of these variables and move 
    //  them into the closure when it is defined. The Once part of the name is
    //  because the closure can’t take ownership of the same variables more than
    //  once, so it can only be called one time.
    // Fn borrows values from the environment immutably.
    // FnMut can change the environment since it mutably borrows values.


impl<T, B, C> Cacher<T, B, C>
    where T: Fn(B) -> C,
          B: Copy + Eq + Hash, 
          C: Copy
{
    //This creates our cacher and initiallizes everything
    fn new(calculation: T) -> Cacher<T, B, C> {
        Cacher {
            calculation,
            value: HashMap::new(),
            // value: None,
        }
    }
    //Instead of calling the closure directly we use the value
    //method to check and see if the value exists already.
    //If it does then it just returns that, but if it doesn't then
    //the closure is evaluated.
    fn value(&mut self, arg: B) -> (){

        // self.value.entry(arg).or_insert((self.calculation)(arg));
        match self.value.get(&arg){
            Some(_) => (),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                ()
            },
        }      
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //Top example shows us calculating the function just once
    //and then using that in sevaral different places
    //  let expensive_result =
        // simulated_expensive_calculation(intensity);

    //This example shows us an example of a closure that does
    //the same calculations as in expensive results.
    //However, a closure is an anonymous function that you
    //can save to a variable or pass as an argument to other
    //functions. You can create the closure in one place and then
    //call it others. They differ from functions in that they are
    //able to capture values from the scope in which they are 
    //called. 
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //An example of a closure with optional type annontation.
    //The compiler is able to infer the types of the closure
    //input and output from what's being inputted. So, it will
    //error if later on you try to do something different from
    //what it inferred.
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //The closure is now defined in one place and only called
    //when it's needed. However, we still run into the problem
    //as from earlier when the expensive function was inside
    //the if statements. The low intensity branch still calls
    //the closure twice.
    //We are now fixing the above with lazy evaluation.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        expensive_result.value(intensity);
        println!(
            "Today, do {} pushups!",
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value.get(&intensity).unwrap()
        );
        //If we uncomment this we can see it doesn't run twice.
        // expensive_result.value(intensity);
        println!(
            "Next, do {} situps!",
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value.get(&intensity).unwrap()
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            expensive_result.value(intensity);
            println!(
                "Today, run for {} minutes!",
                // expensive_result
                // expensive_closure(intensity)
                expensive_result.value.get(&intensity).unwrap()
            );
        }
    }
}

//The below test shows how the above cacher does not update the
//value when a new parameter is being passed into the closure.
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    //We can now see that we've made it generic enough such that it can hold multiple values.
    //The Cacher should also now be generic enough such that the hash map can now return two different
    //types. I could make it more generic such that it doesn't rely on the copy trait but for now
    //I'm happy with the results.

    let mut c = Cacher::new(|a| "a");

    let v1 = c.value(1);
    let v2 = c.value(2);

    println!("{}", c.value.get(&1).unwrap());

    //Here we can see that closures also take in local information about the environment
    //which is something that a function is not capable of doing.
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    // let x = vec![1, 2, 3];
    // An example where we tell the compiler to move the ownership of x to the closure.

    // let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    // let y = vec![1, 2, 3];

    // assert!(equal_to_x(y));
}



