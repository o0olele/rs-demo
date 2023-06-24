#![allow(unused_variables)]

use std::ops::Add;
fn main() {
    print_bad_string();

    slice_example();
    slice_borrow();
    slice_integer();

    string_convert();
    string_index();
    string_push();
    string_replace();
    string_delete();
    string_concatenate();
    string_escape();
    string_utf8();
    string_drop();
}

fn greet(name: String) {
    println!("Hello, {}", name);
}

fn print_bad_string() {
    let name = "ai";

    // greet(name);
}

fn slice_example() {
    let s = String::from("hello rust");

    let hello = &s[0..5];
    let rust = &s[6..10];

    println!("{} {}", hello, rust);

    let slice = rust;
    println!("{:p} {:p}", rust, slice);

    let s = String::from("泪腺战士");
    let slice = &s[..3]; // &s[..2];
    println!("{}", slice);

    println!();
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn slice_borrow() {
    let mut s = String::from("hello ");
    let f = first_word(&s);

    // s.clear();

    println!("{}", f);

    println!();
}

fn slice_integer() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn string_convert() {
    let s1 = String::from("hello");
    let s2 = "world".to_string();

    say_hello(&s1);
    say_hello(&s2);

    say_hello(&s1[1..]);
    say_hello(s1.as_str());

    println!();
}

fn string_index() {
    let s1 = String::from("hello");

    // let h = s1[0];
    // let h = &s1[0];
}

fn string_push() {
    let mut s1 = String::from("hello world");

    s1.insert(5, ',');
    println!("insert -> {}", s1);

    s1.insert_str(6, "you are ");
    println!("insert -> {}", s1);

    println!();
}

fn string_replace() {
    let mut string_replace = String::from("I like rust. Learning rust is my favorite!");

    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    string_replace.replace_range(7..8, "R");
    dbg!(string_replace);

    println!();
}

fn string_delete() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    println!("{}", string_remove.remove(0));
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

fn string_concatenate() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    // let s3 = s1.add(&s2);
    let s3 = s1 + &s2;
    let mut s3 = s3 + "!";
    dbg!(s3);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // println!("{}",s1);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    println!();
}

fn string_escape() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
    can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    println!();
}

fn string_utf8() {
    for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".bytes() {
        println!("{}", b);
    }
}

fn string_drop() {
    {
        let s = String::from("hello"); // 从此处起，s 是有效的
    
        // 使用 s
    }                                  // 此作用域已结束，
                                       // s 不再有效，内存被释放
}