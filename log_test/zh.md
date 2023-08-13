# 运行结果
~~~shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/log_test`
[2023-08-13T10:23:16Z ERROR log_test] 4
~~~

~~~shell
$ cargo run --release
    Finished release [optimized] target(s) in 4.08s
     Running `target/release/log_test`
[2023-08-13T10:23:34Z ERROR log_test] 4
~~~

~~~shell
$ RUST_LOG=log_test cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/log_test`
[2023-08-13T10:22:15Z INFO  log_test] 1
[2023-08-13T10:22:15Z TRACE log_test] 2
[2023-08-13T10:22:15Z DEBUG log_test] 3
[2023-08-13T10:22:15Z ERROR log_test] 4
[2023-08-13T10:22:15Z WARN  log_test] 5
~~~