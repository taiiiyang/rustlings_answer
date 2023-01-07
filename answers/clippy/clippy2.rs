fn main() {
    let mut res = 42;
    let option = Some(12);
    // 使用 if let 做为 Option<T> 的模式匹配
    if let Some(x) = option {
        res += x
    }
    println!("{}", res);
}
