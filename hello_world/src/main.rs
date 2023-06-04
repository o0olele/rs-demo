fn main() {
    greets();
}

fn greets() {
    let a = "hello, world!";
    let b = "i find you.";
    let c = "🙃";

    let lines = [a, b, c];
    for line in lines.iter() {
        println!("{}", line);
    }

    for line in lines {
        println!("{}", line);
    }
}