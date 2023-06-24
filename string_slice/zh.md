# 原文
- [字符串](https://course.rs/basic/compound-type/string-slice.html#%E8%BF%9E%E6%8E%A5-concatenate)

# 运行结果
~~~shell
hello rust
0x564bb5ad9ba6 0x564bb5ad9ba6
泪

h

hello
world
ello
hello

insert -> hello, world
insert -> hello,you are  world

[src/main.rs:113] new_string_replace = "I like RUST. Learning RUST is my favorite!"
[src/main.rs:116] new_string_replacen = "I like RUST. Learning rust is my favorite!"
[src/main.rs:119] string_replace = "I like Rust. Learning rust is my favorite!"

[src/main.rs:128] p1 = Some(
    '!',
)
[src/main.rs:129] p2 = Some(
    '文',
)
[src/main.rs:130] string_pop = "rust pop 中"
string_remove 占 18 个字节
测
[src/main.rs:143] string_remove = "试remove方法"
[src/main.rs:147] string_truncate = "测"
[src/main.rs:151] string_clear = ""
[src/main.rs:161] s3 = "helloworld!"
hello rust!

What are you doing? (\x3F means ?) I'm writing Rust!
Unicode character ℝ (U+211D) is called "DOUBLE-STRUCK CAPITAL R"
String literals
    can span multiple lines.
                        The linebreak and indentation here -><- can be escaped too!
hello \x52\x75\x73\x74
Escapes don't work here: \x3F \u{211D}
And then I said: "There is no escape!"
A string with "# in it. And even "##!

中
国
人
228
184
173
229
155
189
228
186
186
~~~
# 一些问题
- 在`golang`里面，切片实际上是个结构体，那么在`rust`里，它也是个结构体吗？

- string.add
    ~~~c
    let s1 = String::from("hello");
    let s2 = String::from("world");

    // let s3 = s1.add(&s2);
    let s3 = s1 + &s2;
    ~~~

    应该这样理解，比如let s = String::from("I like dogs")，这里并不代表String不能修改，而是s是String的一个不可变的绑定，你可以尝试下后面可以用let mut s1 = s，这个时候s1是String的一个可变的绑定，s1获取了s的所有权，函数调用的使用也是这样，因为获取的是所有权，在获取所有权时你可以决定是可变的绑定(s1)还是不可变的绑定(s)，因为你就是数据的拥有者了，可以自己决定改还是不改。这一点与接受引用的函数是不同的，接受引用其实就是借用，你不能说你是不可变的借用，然后又想要修改数据，所有者不会同意的。