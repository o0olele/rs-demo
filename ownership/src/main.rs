fn main() {
    variable_scope();

    string_type();

    interact_with_move();

    data_copy();

    fn_ownership();

    practice1();
}

fn variable_scope() {
    let s = "str";

    {
        let s = "hello";
        let z = "z";
    }

    println!("{s}");
    // println!("{z}");

    println!()
}

fn string_type() {
    let mut s = String::from("hihi");

    s.push_str(", googoo");

    println!("{s}")
}

fn interact_with_move() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let m = String::from("str");
    let n = m;
    let c = n.clone();

    println!("string = {n}");
    println!("string = {c}");
    // println!("string = {m}");

    println!()
}

fn data_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    println!()
}

fn fn_ownership() {
    let s = String::from("hello");
    take_ownership(s);

    // println!("{}", s);

    let s = String::from("hi");
    let s = take_ownership_and_return(s);
    println!("{}", s);

    let a = 5;
    make_copy(a);
    println!("{a}");

    println!();
}

fn take_ownership(s: String) {
    println!("{}", s)
}

fn take_ownership_and_return(s: String) -> String{
    s
}

fn make_copy(a: i32) {
    println!("{}", a)
}

// practice
fn practice1() {
    let x = Box::new(5);
    
    let mut y = Box::new(1);      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}