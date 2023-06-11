fn main() {
    statement();

    expression();

    assert_eq!(expression_unit(), ());
}

fn statement() {
    let hi = "hi";
    let hi = 4;

    println!("value is {hi}");
    // let b = (let a = 3);

    println!();
}

fn expression() {
    let y = {
        let x = 1;
        x + 1
    };

    // let y = {
    //     y;
    // };

    println!("value is {y}");

    println!();
}

fn expression_unit() {
    let y = if true { "hi" } else { "bye" };
}
