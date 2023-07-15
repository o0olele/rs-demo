fn main() {
    create_struct();
    update_struct();
    struct_in_memory();
    tuple_struct();
    unit_like_struct();
    struct_ownership();
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
}

fn create_struct() {
    let u1 = User {
        name: String::from("kinoko7"),
        email: String::from("kinoko7@gamil.com"),
        sign_in_count: 0,
    };

    // let u1 = User{
    //     name: String::from("kinoko7"),
    //     email:String::from("kinoko7@gamil.com"),
    // };
    // missing field `sign_in_count` in initializer of `User`

    println!("{}", u1.name);

    let mut u2 = User {
        name: String::from("yomyko"),
        email: String::from("yomyko@gamil.com"),
        sign_in_count: 0,
    };
    u2.email = String::from("kinoko7@gamil.com");

    println!("{}", u2.email);

    let u3 = build_user(String::from("api@gmail.com"), String::from("api"));
    let u3 = build_user_bind(String::from("api@gmail.com"), String::from("api"));
    println!("{}", u3.email);

    println!();
}

fn build_user(email: String, name: String) -> User {
    User {
        name: name,
        email: email,
        sign_in_count: 0,
    }
}

fn build_user_bind(email: String, name: String) -> User {
    User {
        name,
        email,
        sign_in_count: 0,
    }
}

fn update_struct() {
    let u1 = User {
        name: String::from("kinoko7"),
        email: String::from("kinoko7@gamil.com"),
        sign_in_count: 0,
    };

    let u2 = User {
        name: u1.name,
        email: u1.email,
        sign_in_count: u1.sign_in_count,
    };

    let u3 = User {
        sign_in_count: 2,
        ..u2
    };

    // println!("{:?} {:?} {:?}", u1, u2, u3); // partial move occurs because `u1.email` has type `String`, which does not implement the `Copy` trait
    // println!("{} {} {}", u1.name, u2.name, u3.name);
    println!(
        "{} {} {}",
        u1.sign_in_count, u2.sign_in_count, u3.sign_in_count
    );

    println!();
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn struct_in_memory() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    println!();
}

struct Color(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("{}", black.0)
}

struct Unit;
trait SomeTrait {
    // ...定义一些行为
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {  }

// 填空，让代码工作
fn do_something_with_unit(u: Unit) { 
    
  }

fn unit_like_struct() {
    let u = Unit;
    do_something_with_unit(u);
}

// struct Owner {
//     username: &str, //missing lifetime specifier
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn struct_ownership() {
    
}