use std::vec;

fn main() {
    // Vector - 动态数组
    {
        // 一种简单声明vector的方法
        let v: Vec<i32> = Vec::new();

        // 如果不想指定类型，那就必须在声明之后写入一个元素
        let mut v = Vec::new(); // unknown type
        v.push(1);

        // 和go一样，你可以预留大小
        let v: Vec<i32> = Vec::with_capacity(10);

        // 还可以使用vec!宏创建vector
        let v = vec![1,2,3];

        // 从vector中读取元素有两种方法：通过下标和使用get
        // 这和C++是完全一致的
        let third = &v[2];
        println!("第三个元素： {}",third);

        // 使用get会更加安全，不过会有轻微的性能损耗
        match v.get(2) {
            Some(third) => println!("第三个元素：{}",third),
            _ => println!("越界了！"),
        }

        // 你可不能在扔掉一个借用之前调用一个需要可变借用的方法
        let mut v = vec![1,2,3];
        let first = &v[0];
        // v.push(4); // invalid!

        // 但你可以drop掉之后push
        drop(first);
        v.push(4);

        // 遍历vec的两种方法
        // 不可变遍历
        let v = vec![1,2,3];
        for i in &v {
            println!("{}",i);
        }

        // 可变遍历
        let mut v  = vec![1,2,3];
        for i in &mut v {
            *i += 1;
        }
    }
}
