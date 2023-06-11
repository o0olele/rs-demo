fn main() {
    let a = 1;
    let b = add(a, 1);

    print_xy(a, b);

    return_unit();

    end();

    looping();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
    // return a + b;
}

fn print_xy(x: i32, y: i32) {
    println!("x = {x}, y = {y}")
}

fn return_unit() -> () {
    
}

fn end() -> ! {
    panic!("system end.")
}

fn looping() {
    loop {
        
    };
}