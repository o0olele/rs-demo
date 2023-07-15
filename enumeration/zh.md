# 原文
- [枚举](https://course.rs/basic/compound-type/enum.html)
- [Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#defining-an-enum)

# 运行结果
~~~shell
Hearts
Diamonds

[src/main.rs:65] c1 = PokerCard {
    suit: Clubs,
    value: 1,
}
[src/main.rs:66] c2 = PokerCard {
    suit: Diamonds,
    value: 12,
}
[src/main.rs:71] c3 = Clubs(
    1,
)
[src/main.rs:72] c4 = Diamonds(
    3,
)
[src/main.rs:77] c5 = Clubs(
    1,
)
[src/main.rs:78] c6 = Diamonds(
    'd',
)

[src/main.rs:92] m = Quit
[src/main.rs:92] m = Move {
    x: 1,
    y: 1,
}
[src/main.rs:92] m = ChangeColor(
    255,
    255,
    0,
)

1

链表的长度是: 3
3, 2, 1, Nil
~~~

# Option<T>
- [What is the difference between Option::None in Rust and null in other languages?](https://stackoverflow.com/questions/73673613/what-is-the-difference-between-optionnone-in-rust-and-null-in-other-languages)