//use crate::List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    //println!("Hello, world!");
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(1, Cons(2, Cons(3, Nil)));
    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    /* *
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    */
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
/* *
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
