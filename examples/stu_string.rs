// rust 字符串学习

//! Rust字符串有以下几种主要类型:
//!
//! - String : 动态字符串,包含完整字符串内容。用{}打印。
//! - &str   : 字符串切片,仅借用字符串不拥有其内容。
//!
//! 打印String类型时需要用"{}",而打印&str类型时不需要!
//!
//! 打印字符串时变量前加&,表明变量是&str切片类型。


fn main() {
    // 声明字符串字面量
    let str = "world";
    println!("测试字符串输出：Hello [{}]", str);

    // 转字符串
    let num = 123.to_string();
    println!("测试数字转字符串：{}", num);

    // 字符串链接
    let s = String::from("Hello ");
    let s = s + "World ";
    let s = s + "!";
    let s = String::from("HH ") + &s; // 这里需要使用 &s 来获取s的值，
    println!("测试字符串拼接：{}", s);

    // let apples = 5;      // 不可变,  let apples = apples + 10;
    // let mut bananas = 5; // 可变, bananas = 20;
    // println!("{}, {}", apples, bananas)
}