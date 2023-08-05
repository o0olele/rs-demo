#![allow(unused)]
fn main() {
    in_function_definitions();
    in_struct_definitions();
    in_enum_definitions();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn in_function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct DiffPoint<T,U> {
    x: T,
    y: U,
}

fn in_struct_definitions() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = DiffPoint{x: 1, y :1.1};

    println!("{:?} {:?} {:?}", integer, float, p);
    println!()
}

fn in_enum_definitions() -> Result<i32, f32> {
    if true {
        return Err(2f32);
    }
    return Ok(1);
}