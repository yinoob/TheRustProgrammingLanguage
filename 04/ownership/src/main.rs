fn main() {
    //println!("Hello, world!");
    /*
    let s = String::from("hello");

    take_ownership(s);

    let x = 5;

    makes_copy(x);
    */
    /*
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    */
    /*
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    */
    /*
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    */

    /*
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
    */

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}
/*
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
*/
/*
fn gives_ownership() -> String {
    let some_string = String::from("Yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

/*
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/
/*
fn calculate_length(s: &String) -> usize {
    s.len()
}
*/

/*
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/
