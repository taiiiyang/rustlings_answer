// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// 暴露该宏
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
