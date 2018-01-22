//Here we're going to show the us of a RefCell which is similar to a
//Box smart pointer except for the fact that while it is an immutable
//object, its content is mutable. The problem with this from the
//compiler stand point is that it can't run the borrowchecker at
//compile time for this object. It instead it is checked at runtime.
//If you violate the borrowchecker then the program will panic on you.

//The below shows the need for a mock objects. So we've let it up to the
//user to implement the send function. We however want to test the
//LimitTracker and its functions. Therefore, we want to implement
//a send function that just that keeps track of the messages being sent.


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
        //We need our sent messages to be a ref cell due to the immutability
        //of the Messenger trait
        // sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
            // MockMessenger { sent_messages: vec![] }
        }
    }
    //We can expect this to give us problems because &self is immutable
    //and we want to mutate it. We can fix this by using a RefCell
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //Here we get a mutable reference to our data
            self.sent_messages.borrow_mut().push(String::from(message));
            // self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        //Here we get an immutable reference to our data
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

//The below will fail at compiler time because we increased our
//count of mutable references to a number greater than 1.
//RefCell keeps track of the number of references that are immutable
//and the number of references that are mutable. It is happy with at most
//one mutable reference. Also, it should be noted that its count of 
//immutable and mutable references go down whenever the ref goes out
//of scope.
//It should be noted that this does impose a small penalty at runtime
//because of it having to keep track of references at runtime.
// impl Messenger for MockMessenger {
//     fn send(&self, message: &str) {
//         let mut one_borrow = self.sent_messages.borrow_mut();
//         let mut two_borrow = self.sent_messages.borrow_mut();

//         one_borrow.push(String::from(message));
//         two_borrow.push(String::from(message));
//     }
// }