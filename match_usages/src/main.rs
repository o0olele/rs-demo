
fn main() {
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     _ => EXPRESSION,
    // }
    let dir = Direct::East;
    match dir {
        Direct::East | Direct::North => println!("east or north"),
        _ => (),
    }

    // if let PATTERN = SOME_VALUE {
    //
    // }
    if let Direct::East = dir {
        println!("east");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while let PATTERN = SOME_VALUE {
    //
    // }
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for PATTERN ..
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    // fn(PATTERN)
    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
enum Direct {
    South,
    West,
    North,
    East,
}
