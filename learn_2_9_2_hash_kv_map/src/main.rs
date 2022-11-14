use std::collections::HashMap;

fn main() {
    // 使用new方法创建一个HashMap
    // 和vector一样，要么在创建的时候指明类型，要么塞一个元素进去
    let mut my_gems = HashMap::new();

    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    // 如果能事先知道kv对个数，HashMap也有capacity方法预留内存
    let my_gems: HashMap<&str, i32> = HashMap::with_capacity(10);

    // 如何将一个Vector转换为一个HashMap？
    // 可以通过迭代Vector（不够rusty）
    let teams_list = vec![
        ("中国队".to_string(), 1),
        ("美国队".to_string(), 1),
        ("日本队".to_string(), 1),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, &team.1);
    }

    // 也可以通过将Vector变成一个iterator，再collect
    // 但因为collect的类型可以有很多，所以要指定是一个HashMap类型，但KV的类型可以自动推导出
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    // 如何查询HashMap里面某个key对应的value？可以通过get方法
    let mut name_score_map: HashMap<&str, i32> = HashMap::new();

    name_score_map.insert("karkli", 100);

    let score = name_score_map.get("karkli");
    match score {
        Some(score) => println!("karkli score {}", score),
        None => println!("karkli has no score"),
    }

    match name_score_map.get("stellahxhu") {
        Some(score) => println!("stellahxhu score {}", score),
        None => println!("stellahxhu has no score"),
    }

    // 还可以循环遍历kv对
    for (key, value) in &name_score_map {
        println!("{}: {}", key, value);
    }

    // 更新可以分覆盖更新和非覆盖更新
    // 覆盖已有的值
    name_score_map.insert("karkli", 10);
    match name_score_map.get("karkli") {
        Some(score) => println!("karkli score {}", score),
        None => println!("karkli has no score"),
    }
    // 不存在就插入新值，并返回新值
    println!(
        "tigajiang val {}",
        name_score_map.entry("tigajiang").or_insert(50)
    );
    // 存在的使用or_insert不会有改变，返回已有值
    println!(
        "karkli val {}",
        name_score_map.entry("karkli").or_insert(25)
    );

    // 你可以为HashMap自定义哈希函数
    // use std::collections::HashMap;
    // use std::hash::BuildHasherDefault;
    // // 引入第三方的哈希函数
    // use twox_hash::XxHash64;

    // // 指定HashMap使用第三方的哈希函数XxHash64
    // let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    // hash.insert(42, "the answer");
    // assert_eq!(hash.get(&42), Some(&"the answer"));
}
