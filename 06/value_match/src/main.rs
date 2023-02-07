#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let third = &v[2];

    println!("The element is {}", third);

    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 33];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", row);

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial content".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    for c in "你好".chars() {
        println!("{}", c);
    }

    for c in "rust".bytes() {
        println!("{}", c);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
