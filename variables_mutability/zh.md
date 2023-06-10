# 原文
- [变量绑定与解构](https://course.rs/basic/variable.html)
- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

# 运行结果
~~~shell
value x is: 1

value x is: 1
value x is: 2

a = true, b = false

1 3 1 4 6

scope value x is: hi
value x is: 2

~~~

# 一些疑问
- 解构式赋值
    ~~~c
    TestE { e, .. } = TestE { e: 5 }; // ?
    ~~~
    这里变量e，和结构体成员同名，但是rust里不允许
    ~~~c
    TestE { e }
    ~~~
    这种写法。
    那么就是`..`的用法？