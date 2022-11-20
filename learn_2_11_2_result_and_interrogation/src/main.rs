fn main() {
    // Result可以用来表达一个结果运行是否成功
    {
        // 假设我们要打开一个文件
        use std::fs::File;

        let f = File::open("hello.txt");
        // let f = match f {
        //     Ok(file) => file,
        //     Err(error) => {
        //         panic!("error when opening hello.txt: {:?}", error);
        //     }
        // };

        // 对所有的error都panic!有点太过残暴了
        use std::io::ErrorKind;
        // 这段代码正常应该创建一个hello.txt，除非你的磁盘已经满了或者不允许创建文件
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(error) => panic!("problem when creating file {:?}", error),
                },
                other_error => panic!("problem when opening file {:?}", other_error),
            },
        };
    }

    // 对Result<T,E>进行模式匹配有点啰嗦
    // 如果你确定要对Err分支进行panic!，那么可以使用unwrap/expect
    // 两者作用一样，后者可以自己写panic信息
    {
        use std::fs::File;

        // let f = File::open("hello2.txt").unwrap(); // 直接panic，不多比比
        // let f = File::open("hello2.txt").expect("failed to open hello2.txt"); // 比比两句
    }

    // 怎么传播错误？就像go那样，if err != nil { return err; }
    {
        use std::fs::File;
        use std::io;
        use std::io::Read;

        // 在所有返回Result的函数后面加一个?符号
        // 代表如果是Err(E)，就作为返回值抛上去，否则继续执行
        fn read_username_from_file() -> Result<String,io::Error> {
            let mut f  = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }

        // ?还可以进行自动类型转换
        // io::Error结构体是实现了std::error::Error trait的，所以可以自动转换
        fn open_file() -> Result<File,Box<dyn std::error::Error>> {
            let mut f = File::open("result.txt")?;
            Ok(f)
        }

        // ?还可以进行链式调用，使得代码看上去quite good
        fn read_username_from_file_v2() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }

        // ? 还可以用在Option里
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }

        // ?有一个前提，就是必须用一个变量承载它，不能直接用于返回
        // fn first(arr: &[i32]) -> Option<i32> {
        //     arr.get(0)? // wrong!
        // }
        fn first(arr: &[i32]) -> Option<i32> {
            let f = arr.get(0)?;
            Some(f.clone())
        }
        fn abs(arr: &[i32]) -> Option<i32> {
            arr.get(0)?.checked_abs()
        }
    }
}
