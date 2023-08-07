#![allow(unused)]
fn main() {
    test_screen();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// T need to be same type, which means all button or all selectbox
// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

fn test_screen() {
    let mut screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    let a = Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("Cancel"),
    });

    screen.components.push(a);

    screen.run();
}

fn draw1(x: &dyn Draw) {
    x.draw();
}
//the size for values of type `(dyn Draw + 'static)` cannot be known at compilation time
// fn draw2(x: dyn Draw) {
//     x.draw();
// }

trait DrawSelf {
    const A: i32 = 1;
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct NullButton;
impl DrawSelf for NullButton {
    fn draw(&self) -> Self {
        return self.clone();
    }
}

// 方法的返回类型不能是 Self
// 方法没有任何泛型参数
// the trait `DrawSelf` cannot be made into an object, consider moving `draw` to another trait
// for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically
// pub struct ScreenSelf {
//     pub components: Vec<Box<dyn DrawSelf>>,
// }

trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> u32 { 42 }
}

impl MyTrait for String {
    fn f(&self) -> String { self.clone() }
}

fn my_function(x: impl MyTrait) -> impl MyTrait  {
    x.f()
}

// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
// }

// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
// }

// fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
//     x.f()
// }
