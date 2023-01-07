use std::mem;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // is_some 判断是否为 Some
    if my_option.is_some() {
        if let Some(x) = my_option {
            // my_option = x
            // do something
        }
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);
    // 使用 vec![] 声明空 vector 同时指明类型
    let my_empty_vec: Vec<String> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // mem::swap 接收两个可变借用
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
