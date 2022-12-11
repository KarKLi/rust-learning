// 一个真实项目中典型的 Package，会包含多个二进制包，
// 这些包文件被放在 src/bin 目录下，每一个文件都是独立的二进制包，同时也会包含一个库包，该包只能存在一个 src/lib.rs
// .
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── main.rs
// │   ├── lib.rs
// │   └── bin
// │       └── main1.rs
// │       └── main2.rs
// ├── tests
// │   └── some_integration_tests.rs
// ├── benches
// │   └── simple_bench.rs
// └── examples
//     └── simple_example.rs

// 唯一库包：src/lib.rs
// 默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
// 其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
// 集成测试文件：tests 目录下
// 基准性能测试 benchmark 文件：benches 目录下
// 项目示例：examples 目录下

fn main() {
    println!("Hello, world!");
}
