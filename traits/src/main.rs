#![allow(unused)]

use std::fmt::Display;
fn main() {
    on_type();
    on_parameter();
    on_returns();

    trait_bound();
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_summarize(&self) -> String {
        "hello".to_string()
    }
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("Title is {}, user is {}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} post a weibo: {}", self.username, self.content)
    }
}

impl std::fmt::Display for Weibo {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

//only traits defined in the current crate can be implemented for types defined outside of the crate
// impl Display for String {

// }

fn on_type() {
    let post = Post {
        title: "Rust ".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is great!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Webo love Tweet!".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    println!("{}", weibo.default_summarize());

    println!();
}

pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn on_parameter() {
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Webo love Tweet!".to_string(),
    };

    notify1(&weibo);

    println!();
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

pub fn notify5(item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(item: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    1
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn trait_bound() {
    let a = Pair { x: 1, y: 2 };

    a.cmp_display();

    println!();
}

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "return return!!",
        )
    }
}

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//         }
//     }
// }

fn on_returns() {
    let a = returns_summarizable();
    println!("{}", a.summarize());

    println!();
}