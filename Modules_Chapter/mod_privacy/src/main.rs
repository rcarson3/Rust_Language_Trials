
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//We can further restrict this if we know we just need one function from a module thus
//further reducing amount of stuff seen in the public scope
// use a::series::of
use a::series::of::nested_modules;
//Here we see an example of how we can bring multiple variants into scope with an enum.
use TrafficLight::{Red, Yellow};
//We can also bring all of the variants into scope at once using the * syntax, which is called the glob operator.
//This brings all of the variants into scope without having to specify them
//use TrafficLight::*;


fn main() {
    // Making calls to nested modules and their functions can get quite lengthy
    // Luckily rust has a better way of doing it
    // a::series::of::nested_modules();
    //Through the use of the use keyword we just bring what we've specified into scope.
    //Therefore, we can reduce the amount of pollution into the public scope.
    // of::nested_modules();
    nested_modules();
    let red = Red;
    let yellow = Yellow;
    //Here we have to specify the TrafficLight namespace because we didn't bring
    //green into scope with the use statement.
    let green = TrafficLight::Green;
}