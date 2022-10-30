fn main() {
    // rust中的字符是一个完整的unicode字符
    {
        let c = 'z';
        let z = 'ℤ';
        let g = '国';
        let heart_eyed_cat = '😻';
        println!("{} {} {} {}", c, z, g, heart_eyed_cat);
        println!("字符'{}'占用了{}的内存大小", g, std::mem::size_of_val(&g));
    }
    // rust中，if后接着的expression只能是一个bool，不能是一个代表零值的类型
    {
        let t = true;
        let a = 1;
        if t { // legal
    
        }
        // if a { // illegal
        // }
    }
}
