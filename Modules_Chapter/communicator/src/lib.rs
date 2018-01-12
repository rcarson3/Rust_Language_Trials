//This means we're declaring module client here but we have it defined in another file.
pub mod client;
//We've moved network to its own file.
pub mod network;

#[cfg(test)]
mod tests {
    //This needs to come at the top before the #[test] 
    use super::client;

    #[test]
    fn it_works() {
        // This fails because the client module is currently not in scope
        // client::connect();
        //We can either use the following to tell Rust to start from the root
        //and then we can list the whole path like this
        //::client::connect();
        //Or we can use the super keyword which tells Rust to look one module up from
        //our current module
        // super::client::connect();
        //However if we have to write super a lot that could get annoying. Therefore, 
        //we can just bring it into scope of the tests module using the use keyword.
        client::connect();
    }
}
