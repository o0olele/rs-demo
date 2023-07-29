fn main() {
    match_example();
    if_let_example();
    matches_example();
    match_var();
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn match_example() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };
    // match dire { //missing match arm: `West` not covered
    //     Direction::East => println!("East"),
    //     Direction::North | Direction::South => {
    //         println!("South or North");
    //     }
    // };

    let a = value_in_cents(Coin::Penny);
    println!("{}", a);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };

    println!();
}

fn if_let_example() {
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    }
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn matches_example() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // let m = v.iter().filter(|x| match x {
    //     MyEnum::Bar => false,
    //     _ => true,
    // });
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", v);

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    println!()
}

fn match_var() {
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let age = Some(30);
    println!("first age {:?}", age);
    match age {
        Some(age) => println!("match age {}", age),
        _ => (),
    }
    println!("later age {:?}", age);
}
