#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    //println!("Hello, world!");
    //let width1 = 30;
    //let height1 = 50;
    /*
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:?}", rect1);
    */
    /* *
    println!(
        "The area of the rectangle is {} square pixles.",
        area(&rect1)
    );
    */
    /* *
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 51,
    };
    /* *
    println!(
        "The area of the rectangle is {} square pixles.",
        rect1.area()
    );
    */

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/
/* *
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/
