fn main() {
    println!("Hello, world!");
}

//Structs are similar to tuples in rust except that we name the variables
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
