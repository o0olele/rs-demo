fn main() {
    enum_value();
    build_enum_struct();
    powerful_enum();
    option_enum();
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
    value: i32
}

fn plus_one(x: Option<&mut WrapInt>) {
    match x {
        None => {},
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
}
