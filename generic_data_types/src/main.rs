#![allow(unused)]
fn main() {
    in_function_definitions();
    in_struct_definitions();
    in_enum_definitions();
    in_method_definitions();
    in_const();

    practice();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn in_function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct DiffPoint<T, U> {
    x: T,
    y: U,
}

fn in_struct_definitions() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = DiffPoint { x: 1, y: 1.1 };

    println!("{:?} {:?} {:?}", integer, float, p);
    println!()
}

fn in_enum_definitions() -> Result<i32, f32> {
    if true {
        return Err(2f32);
    }
    return Ok(1);
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> DiffPoint<T, U> {
    fn mixup<V, W>(self, other: DiffPoint<V, W>) -> DiffPoint<T, W> {
        DiffPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn in_method_definitions() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = DiffPoint { x: 5, y: 10.4 };
    let p2 = DiffPoint { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!();
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn in_const() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
    println!();

    let a= S;
}


// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn practice() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}