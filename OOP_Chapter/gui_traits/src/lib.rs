pub trait Draw {
    fn draw(&self);
}

//Here we can see that we defined a struct that uses a component that contains the trait Draw
pub struct Screen {
    //The Box<Draw> tells us that whatever is stored in Box implements the Draw trait
    pub components: Vec<Box<Draw>>,
}
//Heres an implementation for this trait object
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


//We are now going to implement a couple of types that implement the trait object
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}

//The above is different from the below generic trait object:
//In that in the below case at run time only one type may make up the screen type.
//So let's say we had a button or textfield type. Then if we created screen struct's of the above
//they would all be only of the one kind Screen<Button> or Screen<TextField>. We couldn't have a vec
//that contained a Vec of Box<Button> and Box<TextField> like we can with the above trait object.
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

