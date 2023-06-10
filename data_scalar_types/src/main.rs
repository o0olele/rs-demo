use std::mem;

fn main() {
    annotate_type();

    fn_integer();

    overflow();

    fn_floating_point();

    floating_problem();

    nan();

    operation();

    bit_operation();

    fn_range();
}

fn annotate_type() {
    // let guess = "42e".parse().expect("not a number");
    // let guess = "42_000".parse().expect("not a number");
    let guess: i32 = "42000".parse().expect("not a number");
    println!("value is {:?}", guess);

    println!();
}

fn fn_integer() {
    let i_i8: i8 = 127;
    let i_i16: i16 = 0o44;
    let i_i32: i32 = 0x45f;
    let i_i64: i64 = 100_000;
    let i_i128: i128 = (std::u64::MAX).into();
    let i_isize: isize = 6;

    println!(
        "{} {} {} {} {} {}",
        i_i8, i_i16, i_i32, i_i64, i_i128, i_isize
    );

    println!("{}", mem::size_of::<i8>());
    println!("{}", mem::size_of::<i16>());
    println!("{}", mem::size_of::<i32>());
    println!("{}", mem::size_of::<i64>());
    println!("{}", mem::size_of::<i128>());
    println!("{}", mem::size_of::<isize>());

    println!();
}

fn overflow() {
    let a: u8 = 255;
    let b = a.wrapping_add(10);
    let c = a.checked_add(10);
    let d = a.overflowing_add(10);
    let e = a.saturating_add(10);
    let _f: u8; // = a + 10; // cargo run --release

    println!("{} {} {:?} {:?} {}", a, b, c, d, e);
    println!()
}

fn fn_floating_point() {
    let x = 0.1;
    let y: f32 = 0.1;

    println!("{} {}", x, y);
    println!("{}", x == y.into());
}

fn floating_problem() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
}

fn nan() {
    let a = (-45.0_f32).sqrt();
    if a.is_nan() {
        println!("nan")
    }

    // let b = 1 / 0;
    // println!("{}", b);

    println!();
}

fn operation() {
    let a = 1;
    let b = 2i32;
    let c: i32 = 4;
    let v = a + b + c;

    println!("{a} + {b} + {c} = {v}");

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:.2}", forty_twos[0]);

    println!();
}

fn bit_operation() {
    let a: i32 = 2;
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;

    a <<= b;
    println!("(a << b) value is {}", a);

    println!();
}

struct Test {
    e: i32,
    f: i32
}

fn fn_range() {
    for i in 1..=5 {
        println!("{}", i);
    }

    // Test{e,..} = Test{e:5, f:6}; ?

    println!();
}
