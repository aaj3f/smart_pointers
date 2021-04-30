// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(2, Box::new(Cons(4, Box::new(Nil))));
// }

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// use std::ops::Deref;

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "CustomSmartPointer going out of scope with data: {}",
            self.data
        )
    }
}

fn main() {
    // let x = 5;
    // let y = MyBox::new(5);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let message = MyBox::new(String::from("hellow!"));
    // println!("{}", *message);
    let c = CustomSmartPointer {
        data: String::from("This is something"),
    };
    let d = CustomSmartPointer {
        data: String::from("Sure is!"),
    };
    println!("Just made those two CustomSmartPointers!")
}
