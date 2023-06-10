fn main() {
    let a = 'a';
    let b = ' ';
    let c = '🎶';
    let s = std::mem::size_of_val(&c);

    println!("{a} {b} {c} {s}")
}
