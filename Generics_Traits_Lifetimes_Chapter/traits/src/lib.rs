
//Traits can be thought of as guidelines for the compiler/us that say what must be
//be necessary if a Type were to implement a trait. In this case we are saying that 
//for those implementing Summarizable they must have a function called summary that takes
//in a borrow of itself and then returns a string.
//A trait can have multiple methods in its body, with the method signatures
//listed one per line and each line ending in a semicolon.

pub mod aggregate{
    pub trait Summarizable {
        //The semicolon at the end here is necessary for traits.
        // fn summary(&self) -> String;
        //This shows us to implement a default default implementation of a trait
        //if it isn't specified for something.

        //Default methods are able to call other methods in the same trait
        //even if those methods don't have a default implementations. A trait can
        //provide a lot of useful functionality and only require implementors to 
        //specify a small part of it.
        fn author_summary(&self) -> String;

        fn summary(&self) -> String {
            format!("(Read more from {}...)", self.author_summary())
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    //The for after the trait name is necessary so that the compiler knows what
    //type we want to implement the trait for
    impl Summarizable for NewsArticle {
        fn author_summary(&self) -> String {
            format!("{}", self.author)
        }
    }

    // impl Summarizable for NewsArticle {
    //     fn summary(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        // fn summary(&self) -> String {
        //     format!("{}: {}", self.username, self.content)
        // }

        //An example of how we can use the default summary by just defining the
        //author summary
        //However, if we define summary already then we can't use the default
        //summary implementation
        fn author_summary(&self) -> String {
            format!("@{}", self.username)
        }
    }
}

