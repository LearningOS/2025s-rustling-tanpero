// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 问题1：在检查 is_none() 后调用 unwrap() 是不安全的
    // 修复：移除这个不安全的调用或使用 if let 模式
    if my_option.is_none() {
        // 不应该在知道是 None 的情况下调用 unwrap()
        // my_option.unwrap(); - 删除这行
    }

    // 问题2：数组定义中缺少逗号
    let my_arr = &[
        -1, -2, -3,  // 添加逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 问题3：resize 方法不返回值，而是修改原向量
    // 修复：分开声明和调用 resize
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 问题4：交换变量的值应该使用 std::mem::swap
    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
