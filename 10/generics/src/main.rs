fn main() {
    //println!("Hello, world!");
    /* 获取列表中最大的值*
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    */

    let number_list = vec![34, 50, 35, 100, 56];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 43, 3333, 2222, 46];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
//用函数重写获取列表中最大的值
/* *
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/
//用泛型重写获取列表中最大的值
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
