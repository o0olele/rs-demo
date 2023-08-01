fn main() {
    literals();
    named_variables();
    multiple_patterns();
    with_range();

    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_enums();
    destructuring_structs_tuples();
    destructuring_array();

    ignore_match();

    match_guard();
    at_match();
}

fn literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!();
}

fn named_variables() {
    let x = Some(5);
    let y = 10;
    // let x = Some(50);

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
    println!();
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!();
}

fn with_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    println!();
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    // let Point { a, b } = p; //pattern does not mention fields `x`, `y`
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        // Message::Quit{} => {
        //     println!("quit ");
        // }
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x: 1, y } => {
            println!("Move in the x direction 0 and in the y direction {y}");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    println!();
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_nested_structs_enums() {
    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    println!();
}

fn destructuring_structs_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{} {} {} {}", feet, inches, x, y);

    println!();
}

fn destructuring_array() {
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;

    assert_eq!(x, 114);
    assert_eq!(y, 514);

    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        println!("{:?} {:p} {:p}", x, x, &114);
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }

    let arr: &[u16] = &[];

    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));
}

fn ignore_match() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // match s {
    //     Some(_c) => println!("found a string"),
    //     _ => ()
    // }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    println!()
}

fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(x) if x == y => println!("Matched, n = {}", x),
        // Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    println!()
}

enum HiMessage {
    Hello { id: i32 },
    World { name: String}
}
fn at_match() {
    let msg = HiMessage::Hello { id: 5 };

    match msg {
        HiMessage::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        HiMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
            // println!("Found an id in another range {}", id)
        },
        HiMessage::Hello { id } => {
            println!("Found some other id: {}", id)
        },
        _ => ()
    }

    // let msg = HiMessage::World { name: String::from("world") };
    // match msg {
    //     HiMessage::World { name:  } => {

    //     },
    //     _ => ()
    // }
}