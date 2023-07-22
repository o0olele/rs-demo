fn main() {
    if_expressions();
    for_repetition();
    while_repetition();
    loop_repetition();
}

fn if_expressions() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!();
}

#[derive(Debug)]
struct User {
    name: String,
}

fn for_repetition() {
    for i in 1..=5 {
        // [1,5]
        println!("{}", i);
    }

    let u32_list: [u32; 5] = [1, 2, 3, 4, 5];
    for val in u32_list {
        println!("{}", val);
    }
    for val in &u32_list {
        println!("{}", val);
    }

    let user_list: [User; 2] = [
        User {
            name: String::from("kinoko7"),
        },
        User {
            name: String::from("yommyko"),
        },
    ];
    // for val in user_list {
    //     println!("{}", val.name)
    // }
    // println!("{:?}", user_list) // borrow of moved value: `user_list`
    for val in &user_list {
        println!("{}", val.name);
        // val.name = String::from("ai"); // cannot assign to `val.name`, which is behind a `&` reference `val` is a `&` reference, so the data it refers to cannot be written
    }
    println!("{:?}", user_list);

    let mut user_list: [User; 2] = [
        User {
            name: String::from("kinoko7"),
        },
        User {
            name: String::from("yommyko"),
        },
    ];
    for val in &mut user_list {
        val.name = String::from("ai");
    }

    let arr = [11, 22, 33, 44];
    for (i, v) in arr.iter().enumerate() {
        println!("{} = {}", i, v);
    }

    for _ in 0..10 {
        println!("repeat 10 times.")
    }
    for i in 0..10 {
        _ = i;
    }

    println!();
}

fn while_repetition() {
    let mut n = 0;
    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    println!();
}

fn loop_repetition() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            if count == 3 {
                continue 'counting_up; 
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!();
}
