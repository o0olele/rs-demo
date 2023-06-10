fn main() {
    bad_immutable();

    immutable();

    underline_variables();

    deconstruction();

    deconstruction_assign();

    consts();

    shadowing();

    bad_shadowing();
}

fn bad_immutable() {
    let x = 1;
    println!("value x is: {x}");

    // x = 2;
    // println!("value x is: {x}");

    println!();
}

fn immutable() {
    let mut x = 1;
    println!("value x is: {x}");

    x = 2;
    println!("value x is: {x}");

    println!();
}

fn underline_variables() {
    let _x = 1;
    // let y = 2; // can use quick fix in vscode
    let _y = 2;
}

fn deconstruction() {
    let (a, mut b) = (true, false);
    println!("a = {a}, b = {b}");

    b = true;
    assert_eq!(a, b);

    println!();
}

struct TestF {
    f: i32,
}

struct TestE {
    e: i32
}


fn deconstruction_assign() {
    let (a, mut b, c, d, mut e);

    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];

    b = 3;

    TestE { e, .. } = TestE { e: 5 }; // ?
    TestF { f: e } = TestF { f: 6 };

    println!("{} {} {} {} {}", a, b, c, d, e);

    println!()
}

fn consts() {
    const M: i32 = 100_000;
    const m: i32 = 10;
}

fn shadowing() {
    let x = 1;

    let x = 2;

    {
        let x = "hi";
        println!("scope value x is: {x}");
    }

    println!("value x is: {x}");

    println!();
}

fn bad_shadowing() {
    let mut x = "hi";
    // x = x.len();
}