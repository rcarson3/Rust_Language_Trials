fn main() {
    //Iterators in Rust are lazy. In other words, they
    //have no effect unitl we call methods that consume
    //the iterator.
    //Iterators all implement a trait called Iterator defined in the
    //std lib.

    let v1: Vec<i32> = vec![1, 2, 3];
    //Other methods defined on the Iterator trait are called iterator
    //adaptors, and they allow us to change iterators into different
    //kinds of iterators.
    //Since iterators are lazy we have to call a consuming method
    //to get results from the iterator.
    //The map iterator adaptor takes in a closure to call on each item
    //in order to produce a new iterator.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
//We are now going to create our own implementation of the Iterator
//trait for our specific data structure.
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    //Here we're just specifying that the returned item is of type u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        //We only want this to iterate from 1 to 5
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

//Here we can test and see that it only counts to 5 and then returns
//none.
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

//This test shows that since we implemented the next method we now have
//access to a lot of other iterator adaptors that are built into the 
//stdlib.
#[test]
fn using_other_iterator_trait_methods() {
    //This is an example where we take a counter and pair it with
    //another counter that we've skipped the 1st value. Then we 
    //multiply each pair together and only keep the results of those
    //that are divisible by 3. Then we sum together the results of all
    //that. It should be noted that zip returns None when either of
    //its inputs return None.
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}

//The requirements for an iterator trait.
// trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    //The values we get from next are immutable references to the
    //the values. So, iter provides immutable references
    //to the values over whatever we're iterating over.
    //If we want our iterator to take ownership then we call
    // into_iter, and if we want mutable references we call iter_mut
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //Methods that call next are called consumming adaptors because they
    //use up the iterator. One such example is the sum method.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

//The below shows a common use of a closure that captures their env
//by using the filter iterator adaptor.
//The filter adaptor only returns values to the iterator that are true.
//If the values are returned false then the values won't be included
//in the resulting iterator.
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
//This function takes ownership of the vector being passed into it.
//Since we call into_iter over the vector the resulting iterator takes
//ownership of the data. The output of this is the owned data
//that matches the filter requirements.
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
