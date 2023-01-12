// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

fn main() {
    my_macro!();
}
// 只要导入了定义这个宏的crate，该宏就是可用的
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
