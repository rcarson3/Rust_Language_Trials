pub mod outermost{
    pub fn middle_function(){}

    fn middle_secret_function(){}

    pub mod inside {
        pub fn inner_function(){
            ::outermost::middle_secret_function();
            }
        fn secret_function(){}
    }
}
//If a function is in the same root module that the private module is in then 
//the function may use that modules functions. However, it may not have access
//to any children modules that are within that private module.
//Middle secret function may only be accessed by function inside outermost or by
//functions that are within any children modules of outermost
fn try_me(){
    outermost::middle_function(); //this works because it's public
    // outermost::middle_secret_function(); // this fails because it's private
    outermost::inside::inner_function(); // this fails because mod inside is private
}