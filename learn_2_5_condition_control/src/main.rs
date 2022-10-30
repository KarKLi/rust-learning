fn main() {
    // if 语块是表达式，所以可以使用if语句给变量赋值
    {
        let cond = true;
        let number = if cond { 5 } else { 6 };
        // 因为if里面的语块是表达式，所以if-else返回的值都必须是相同类型
        // let anything = if cond { "Hello" } else { 1 }; // wrong!
    }
    // for简单好用，常用for..in...的形式
    {
        for i in 1..=5 {
            // 循环[1,5]
        }
        for i in 1..5 {
            // 循环[1,5)
        }
        // for 语句会夺走待遍历没有实现copy trait的集合的所有权，如果你只是想遍历它，使用它的借用
        let a = vec![1, 2, 3];
        for i in a {}
        // a 已经无效了
        // for i in a {} // invalid!

        // 使用借用
        let mut a = vec![1, 2, 3];
        for i in &a {} // valid
        for i in &mut a {} // 在循环中希望修改元素需要加上&mut关键字

        // 而定长数组实现了copy trait，直接遍历会存在拷贝，因此不需要使用借用
        let a = [1, 2, 3];
        for i in a {}
        for i in a {} // still valid

        // 如何循环时获取索引呢？像python一样使用enumerate
        for (i, v) in [1, 2, 3, 4].iter().enumerate() {}

        // 如果只是想循环十次，应该怎样呢？
        for _ in 1..10 {}
        // 还可以使用while，但是需要事先声明一个mut变量
        let mut n = 0;
        while n <= 10 {n+=1;}
    }
    // 死循环有一个简单好用的关键字loop
    {
        let mut n = 0;
        loop {
            n += 1;
            if n > 5 {
                break
            }
        } // 无限循环！（如果你忽略里面的break语句）

        // break 还可以带上一个值作为返回值（这说明loop也是一个语句！）
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter >= 5 {
                break counter; // 就像return一样，只是return用在函数，break用在for，while和loop
            }
        };
    }
}
