fn main() {
    assert_eq!((), func());
    println!("{}", std::mem::size_of_val(&func()))
}

fn func() {
    
}