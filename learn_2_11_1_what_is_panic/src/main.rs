fn main() {
    // 什么是panic!？
    // panic!是标准库提供的一个宏，用于终止程序
    {
        // panic!("hello, world!");
    }

    // panic可以使用backtrace展开，下面的例子可以在linux通过设置RUST_BACKTRACE=1来展开调用栈
    {
        // let v = vec![1, 2, 3];
        // v[99];

        // 会出现这样的调用栈
        //         thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:11:9
        // stack backtrace:
        //    0: rust_begin_unwind
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:584:5
        //    1: core::panicking::panic_fmt
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/panicking.rs:142:14
        //    2: core::panicking::panic_bounds_check
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/panicking.rs:84:5
        //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/slice/index.rs:242:10
        //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/slice/index.rs:18:9
        //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/alloc/src/vec/mod.rs:2627:9
        //    6: learn_2_11_1_what_is_panic::main
        //              at ./src/main.rs:11:9
        //    7: core::ops::function::FnOnce::call_once
        //              at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/ops/function.rs:248:5
        // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    }

    // panic默认是采用栈展开的模式来终止程序的，但你如果并不关心调用栈问题是啥，你可以在cargo.toml中配置这个来直接终止
    //     [profile.release]
    // panic = 'abort'

    // 子线程panic不会导致父线程panic，这点和go不同

    // 使用panic的例子：unwrap
    {
        use std::net::IpAddr;
        // parse函数会抛出一个Result<T,E>，如果使用unwrap，则会直接返回T
        // 那E怎么办呢？如果Result<T,E>实际是个Err(E)，unwrap直接panic!就完事儿了
        let home: IpAddr = "127.0.0.1".parse().unwrap(); // no panic

        // let home: IpAddr = "127.256.0.1".parse().unwrap(); // panic!
    }

    // 谨记使用panic!的原则：当错误无法恢复或者让业务完全无法往下走的时候，panic!
    // 一般情况下在业务函数里禁止使用panic!
}
