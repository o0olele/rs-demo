fn main() {
    build_array();
    index_array();
    slice_array();
    array_example();
}

// fix length array allocate in stack, not in heap
fn build_array() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [3; 5];
}

fn index_array() {
    let a = [9, 8, 7, 6, 5];

    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素

    // println!("{}", a[first]); panic
    println!("{} {}", first, second);
}

fn slice_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn array_example() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
