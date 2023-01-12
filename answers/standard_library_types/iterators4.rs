// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // (1, 2) => 2
    // (2, 3) => 6
    // (6, 4) => 24
    // (24, 5) => 120
    // fold() 相当于 reducer 接受一个初始值和闭包，将初始值先传入，
    // 再将迭代器的第一个值作为第二个参数，返回值作为新的第一个参数，
    // 当迭代器完成之后，返回最后一个数
    (2..=num).fold(1, |x, b| {
        println!("{x}, {b}");
        x * b
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
