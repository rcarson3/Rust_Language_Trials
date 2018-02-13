// The book “Design Patterns: Elements of Reusable Object-Oriented Software,” colloquially referred to 
// as “The Gang of Four book,” is a catalog of object-oriented design patterns. 
// It defines object-oriented programming in this way:

    // Object-oriented programs are made up of objects. An object packages both data and the procedures that 
    // operate on that data. The procedures are typically called methods or operations.

// Therefore Rust is object oriented since structs and enums have data and
// impl blocks provide implementations. Rust also allows us to encapsulate data which is another common idea
//in object orient languages. 

//If an object orientated language must have inherentince for it to be one than Rust does not fit this definition.
//It does have the trait system which enables a form of polymorphism in Rust. Rust's generics also set up a form of
//polymorphism called bounded parametric polymorphism.

pub struct AveragedCollection {
    //List and average are kept private
    //We don't want people messing with these externally
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    //The user has to use our public signatures to add or remove data
    //Then if they want the average they have to use the public interface
    //By setting up our code this way, we can change the internals and not
    //cause external codes to break.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    //You can access the average here
    pub fn average(&self) -> f64 {
        self.average
    }
    //This function is kept private.
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}