
//HashMaps aren't used nearly as often and so they aren't automatically
//brought into scope. Therefore, we need to manually call them in.
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;
    //Hash maps store keys associated with values. We can think of them like
    //dictionaries in python.

    //Just like Vectors HashMaps are constructed on the heap.
    let mut scores = HashMap::new();

    //Hash maps require a homogenous data structure. Keys must all be one type
    //and the values must all be of one kind.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Hash maps can also be constructed using the collect method as seen below.


    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    //The zip method allows us to create a vector of tuples  where "Blue" is
    //paired with 10 and so forth.
    //The type annotation HashMap<_, _> is needed because it's possible to collect
    //into many different data structures and Rust doesn't know what data type you want.
    //The _, _ is simply used because Rust can infer what types those should be from
    //the types of the data in the vectors.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //Hash maps and ownerships:
    //For types that have the Copy trait the Hash Map simply retains a copy of those values.
    //When that is not the case the hash map now gains control of the ownership of the data.
    //However, if we insert references to the data into the hash map then the hash map does
    //not own the data. The big caveat to this is that reference must be valid for the 
    //complete life of the hash map.

    //We can get values out of a hash map by doing the following:


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    //score is returned as Some(10) because if there is no value associated with that key then
    //a None value is returned. 
    println!("Score is {:?}", score);

    //We can also iterate over the hash map the same way we might with a vector.

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //If we want to update a hash map we can do several things: 1)Overwrite the old value with the new value,
    //2) keep the old value if there is one and disregard the new if an old value exists, or 3) we combine
    //both the old and new values

    //Here we just rewrite the old value with the new value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Here we check to see if a key has no value.
    //So hash maps have a special api called entry that takes in a key and returns an Enum called Entry.
    //This enum represents if the value exists or not. The or_insert method on Entry is defined to
    //return the value corresponding to the Entry key if it exists, else it inserts the parameter
    //as the new value for this key.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    //The key "Blue" already has an entry so it isn't updated
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //Another common thing with hash maps is to look up a key's value and then update it based on
    //the old value. The below is an example of this:

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //So or_insert returns a mutable reference to the value
        let count = map.entry(word).or_insert(0);
        //We update this reference here by dereferencing count and then
        //incrementing the count by one. 
        *count += 1;
    }//Once this ends the mutable reference to count goes away so everything is safe.

    println!("{:?}", map);

    //So HashMap uses a cryptographically secure hashing function that can provide some resistance to DoS attacks.
    //Since this is not the fastest hashing function you can swap this out with another one that implements the
    //BuildHasher trait if you need either something faster or something even more secure.

}
