fn main() {
    // 和C++中的std::array一样，rust的数组也是定长的
    {
        let a = [1, 2, 3, 4, 5];
        // a的类型为[i32;5]，代表i32类型，长度为5，因此，也可以写成：
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        // 如果要生成重复元素的数组，语法是：[值;重复次数]
        let a = [3;5]; // [3,3,3,3,3]
    }
    // rust的数组因为是定长的，所以是在栈上分配的，但这和数组本身存什么内容没有关系
    {
        // String是在堆上分配的，但仍然可以作为定长数组的元素类型
        let a = [String::from("hello")];
    }
    // 数组可以通过下标访问元素
    {
        let a = [9,8,7,6,5];
        let first = a[0];
        let second = a[1];
        // 越界会panic
        // let sixth = a[6]; // panic!也无法通过编译
    }
    // 数组可以进行切片
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
        assert_eq!(&[2,3],slice);
    }
    // 定长数组可以直接用做循环对象，也可以生成一个迭代器
    {
        let a = [1,2,3,4,5];
        for n in a {
            println!("{n}");
        }
        for n in a.iter() {
            println!("{n}");
        }
        for i in 0..a.len() {
            println!("{}",a[i]);
        }
    }
}
