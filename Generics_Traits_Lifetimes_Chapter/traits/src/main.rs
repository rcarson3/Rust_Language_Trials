extern crate traits;

//You have to also call in the trait as well from the crate
//so you can't just have the struct being brought into scope
use traits::aggregate::Tweet;
use traits::aggregate::NewsArticle;
use traits::aggregate::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}
//One restrict to trait implementations. You can implement a trait on a type
//as long as the trait or type are local to our crate.
//You can't implement an external trait on an external type. The reason behind this is
//that you don't want two crates implementing the same trait on the same type and having
//the traits conflict with each other. Rust wouldn't know then what trait to use.
impl Summarizable for WeatherForecast {
    fn author_summary(&self) -> String {
            format!("N/A")
    }

    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}

//This is called trait bounding. We can tell the compiler that we are bounding the
//types to only those that implement this trait.
pub fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}

//One last thing we can also conditionally implement methods based upon the trait
//bounds as seen below:

//Based on all of this we can now create code that is generic but still requires
//certain features to be implemented. Therefore, we can get rid of what would be runtime
//errors in other languages at compile time. For example, if we tried to create a generic
//implementation that had some trait required, but we make this implementation that
//didn't have that trait it would be caught at compile time. This also allows us
//to get some improved performance since certain things are now guaranteed at compile
//time.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//When a function has multiple generic types and their own trait bounds we
//use the following syntax:
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
//where T: Display + Clone  means to use only the types that have these traits
//defined already in them
//The above method is pretty messy so Rust has a nice way of writing this as the
//following.
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    notify(&article);
    notify(&tweet);

    println!("New article available! {}", article.summary());

    println!("1 new tweet: {}", tweet.summary());
}
