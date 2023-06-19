#![allow(unused_variables)]
fn main() {
    print_bad_string();

    slice_example();
    slice_borrow();
    slice_integer();
}

fn greet(name: String) {
    println!("Hello, {}", name);
}

fn print_bad_string() {
    let name = "ai";

    // greet(name);
}

fn slice_example() {
    let s = String::from("hello rust");

    let hello = &s[0..5];
    let rust = &s[6..10];

    println!("{} {}", hello, rust);

    let slice = rust;
    println!("{:p} {:p}", rust, slice);

    let s = String::from("泪腺战士");
    let slice = &s[..3]; // &s[..2];
    println!("{}", slice);

    println!();
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn slice_borrow() {
    let mut s = String::from("hello ");
    let f = first_word(&s);

    // s.clear();

    println!("{}", f);

    println!();
}

fn slice_integer() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
