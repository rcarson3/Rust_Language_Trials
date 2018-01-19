//cargo test -- --test-threads=1 here is an example of how to run tests in serial
//  cargo test -- --nocapture allows us to see printed values for the passed tests as
//well
//cargo test -- --ignored let's you run all of the tests that are normally ignored
//You can run a specific test by say cargo test name_of_fcn
//You can run several tests by giving part of a name so that all tests that
//contain that name will be run and this includes modules since their name
//becomes part of all of the test names.

//The #[cfg(test)] tells the compiler to only compile the test when we run cargo test
//You should really just do this with unit tests instead of integration tests
//Since in integrations your checking that everything works together instead of small
//units of the tests.
//If we want to do integration tests you need to make a tests directory. Cargo
//will then know that integration tests are in there.
#[cfg(test)]
mod tests {
    //This line tells us that this is a test function
    // #[test]
    // fn exploration() {
    //     assert_eq!(2 + 2, 4);
    // }
    // #[test]
    // An example to see a test that fails when a panic occurs
    // fn another(){
    //     panic!("Make this test fail");
    // }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        //Here we're enforcing that our boolean is returned correctly
        assert!(larger.can_hold(&smaller));
    }

    //Since we added the ignore tag here it won't run unless we explicitly tell it to.
    #[test]
    #[ignore]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        //Here we're enforcing that a smaller rect can't hold a bigger one
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        //Here is an example of the assert_eq macro
        assert_eq!(4, add_two(2));
        // The assert_ne macro is useful when you aren't sure what the answer should be
        // but you know what it can't be.
        // assert_ne!();
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Generic assert that doesn't output any really useful info about the failure
        // assert!(result.contains("Carol"));
        // More useful version because we customized the text of it.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    //Here's an example where we are testing that our code should fail when we expect it
    //to and panic.
    //We can make should panic more specific by adding (expected = " Insert some string here")
    //where the inserted string is part of the panic message.
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        //Here's to show what happens when the panic message isn't one that we
        //expect it to be
        // Guess::new(0);
        Guess::new(200);
    }
}


pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

pub fn greeting(name: &str) -> String {
    //Correct version
    format!("Hello {}!", name)
    // Incorrect version
    // String::from("Hello!")
}

pub fn add_two(a: i32) -> i32 {
    //Here we inserted a bug to show that it failed
    // a + 3
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
        // Bug that we're inputting for the tests to fail
        // self.length < other.length && self.width > other.width
    }
}
