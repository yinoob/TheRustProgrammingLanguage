use std::io;
fn main() {
    /*
    let mut x = 5;
    println!("The value of is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    */
    //println!("Hello, world!");
    /*
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    //println!("The value of x is: {}", x);
    //println!("The value of y is: {}", y);
    //println!("The value of z is: {}", z);
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);
    */

    let a = [1, 2, 3, 4, 5];

    println!("please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
