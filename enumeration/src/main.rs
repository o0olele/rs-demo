fn main() {
    enum_value();
    build_enum_struct();
    powerful_enum();
    option_enum();
    list_enum();
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

fn enum_value() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    // println!("{:?}", heart);

    println!();
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

#[derive(Debug)]
enum OptPokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
enum DiffPokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn build_enum_struct() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    dbg!(c1);
    dbg!(c2);

    let c3 = OptPokerCard::Clubs(1);
    let c4 = OptPokerCard::Diamonds(3);

    dbg!(c3);
    dbg!(c4);

    let c5 = DiffPokerCard::Clubs(1);
    let c6 = DiffPokerCard::Diamonds('d');

    dbg!(c5);
    dbg!(c6);

    println!();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn handle_message(m: Message) {
    dbg!(m);
}

fn powerful_enum() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);

    handle_message(m1);
    handle_message(m2);
    handle_message(m3);

    println!();
}

struct WrapInt {
    value: i32,
}

fn plus_one(x: Option<&mut WrapInt>) {
    match x {
        None => {}
        Some(i) => {
            i.value = 1;
            return;
        }
    };
}

fn option_enum() {
    let a = &mut WrapInt { value: 9 };
    plus_one(Some(a));

    println!("{}", a.value);
    println!();
}

// 填空，让代码运行
use crate::List::*;

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0,
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn list_enum() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}
