
//When the module has a declaration of another module in it, we can not have it wrapped inside of a main call. This
//worked for the other cases, but it won't work here. 

    //Everything inside of here is in the name space of the module
    //If we wanted to call this function from a script outside the network module, we would need to 
    //specify the module and use the namespace syntax ::, like so: network::connect() rather than just connect().
    // fn connect(){
    // }
    // //If we want to move this to its own file we have to either make this it's own module instead of a child module.
    // //Or we can make a network directory rename network.rs to mod.rs and then we can declare the module like
    // //we used the old ways but keeping it in the network directory.
    // // mod server{
    // //     fn connect(){
    // //     }
    // // }
    mod server;

    pub fn connect() {
        server::connect();
    }
    // //We can have modules within modules. The choice is up to the programmer and how they see different
    // //relationships between different parts of code.
    // //Client is a child of network instead of being a sibling if it was outside the module block for
    // //network.
    // mod client{
    //     //Even though these functions have exactly the same name they do not conflict with each other.
    //     //This is due to the fact that both of them are in different modules
    //     fn connect(){
    //     }
    // }