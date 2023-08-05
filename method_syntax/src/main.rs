#![allow(unused)]
fn main() {
    defining_methods();
}

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    // &self = self: &Self
    pub fn center(self: &Self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Circle {
    pub fn in_circle(&self, other: &Circle) -> bool {
        true
    }
}

/*
class File {
    Data
    Methods
}

in rust
struct File {
    Data
}

impl File {
    Methods
}
*/
fn defining_methods() {
    let a = Circle::new(0.1, 0.3, 5.0);

    println!("circle center is {:?}", a.center());
    println!("circle area is {}", a.area());
    println!("circle center x is {}", a.x);

    let b = Circle::new(1f64, 2f64, 2f64);
    let ok = (&a).in_circle(&b);
    let ok = a.in_circle(&b);

    println!();
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}