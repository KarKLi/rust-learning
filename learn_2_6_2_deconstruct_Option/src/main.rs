fn main() {
    // Some和None都是Option enum的成员，因此在匹配一个Option的时候，你完全可以在不同的模式返回Some或None
    let mut a = Some(3);
    a = match a {
        Some(temp) => Some(temp + 1),
        None => None,
    };
    println!("after match, a is {:?}", a);
}
