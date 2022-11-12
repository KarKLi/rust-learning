fn main() {
    // 不仅if let可以匹配单种模式，还可以使用while let匹配单种模式
    {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            // 这里会一直弹到没有为止，没有的话，stack.pop()就为None
            println!("{}", top);
        }
    }
    // for循环也可以使用模式匹配解构index和value
    {
        let v = vec!['a', 'b', 'c'];
        for (idx, val) in v.iter().enumerate() {
            println!("{} is at index{}", val, idx);
        }
    }
    // let也是一种模式匹配，因此你可以使用以下的语法解构数据
    {
        let (x, y, z) = (1, 2, 3); // 如果两边的类型不同，那模式是不匹配的，自然也就无法解构了
        println!("{}, {}, {}", x, y, z);
    }
    // 函数参数也是一种模式，因此你可以写出这种骚代码
    {
        fn print_coor(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coor(&point);
    }
}
