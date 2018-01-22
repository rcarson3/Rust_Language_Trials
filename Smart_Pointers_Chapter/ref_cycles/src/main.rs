use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
//Note that list now contains a RefCell that contains a Rc
//This means that it is possible for us to create a cyclic loop.
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
//By implementing a tail function, it's possible to create
//a memory leak in rust.
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}


#[derive(Debug)]
struct Node {
    value: i32,
    //The Weak type allows a Rc to be cleaned even if there are
    //a number of reference counts to weak types. We'll see
    //how it's used further down below. However, it should be 
    //noted here that we can check those with a weak type
    //to see if their parent still exists.
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // //Here we can see that we are linking the two together such that
    // //the tail end of a now points to b which has a tail that
    // //points to a.
    // if let Some(ref link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        //Initially we say that the leaf has no parent or children
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //Here we are creating what will become the parent to leaf
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        //We now say that the branch has a child called leaf
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    //Since leaf was not aware that it is now a child, we need to 
    //modify it's parent value to be referenced to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    //We can now check and see how weak count and strong count change
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        //Here we can see that the branch's weak count increased by
        //one and not leaf's
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


}