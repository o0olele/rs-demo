# 原文
- [流程控制](https://course.rs/basic/flow-control.html)
- [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

# 运行结果
~~~shell
The value of number is: 5
number is divisible by 3

1
2
3
4
5
1
2
3
4
5
1
2
3
4
5
kinoko7
yommyko
[User { name: "kinoko7" }, User { name: "yommyko" }]
0 = 11
1 = 22
2 = 33
3 = 44
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.
repeat 10 times.

0!
1!
2!
3!
4!
5!
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50

The result is 20
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2

~~~

# 注意事项
- 第一种方式是循环索引，然后通过索引下标去访问集合，第二种方式是直接循环集合中的元素，优劣如下：
    - 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    - 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)。