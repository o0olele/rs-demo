fn main() {
    references();

    immutable_reference();

    mutable_reference();

    dangling_reference();

    practice1();
}

fn references() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    println!("{:p}", y);

    println!();
}

fn calc_length(s: &String) -> usize {
    return s.len();
}

// fn change(s: &String) {
//     s.push_str("hi");
// }

fn immutable_reference() {
    let str = String::from("value");
    let size = calc_length(&str);

    println!("string {str} length {size}");

    println!();
}

fn change(s: &mut String) {
    s.push_str("hi");
}

fn mutable_reference() {
    let mut str = String::from("value ");

    change(&mut str);

    println!("{str}");

    // let r1 = &mut str;
    // let r2 = &mut str;
    // println!("{} {}", r1, r2);

    {
        let r1 = &mut str;
    }
    let r2 = &mut str;
    println!("{}", r2);

    // let r3 = &str;
    // let r4 = &mut str;
    // println!("{} {}", r3, r4);

    let r3 = &str;
    println!("{}", r3);
    let r4 = &mut str;
    println!("{}", r4);

    println!();
}

// fn dangle() -> &String {
//     let s = String::from("value");

//     &s
// }

fn no_dangle() -> String{
    let s = String::from("value");

    s
}

fn dangling_reference() {
    // let reference_to_nothing = dangle();
    let str = no_dangle();

    println!("{str}");

    println!();
}

fn practice1() {
    let c = '中';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);
}