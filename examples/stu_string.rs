/*
 * MIT License
 *
 * Copyright (c) 2023 tomoncle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// rust 字符串学习

//! Rust字符串有以下几种主要类型:
//!
//! - String : 动态字符串,包含完整字符串内容。用{}打印。
//! - &str   : 字符串切片,仅借用字符串不拥有其内容。
//!
//! 打印String类型时需要用"{}",而打印&str类型时不需要!
//!
//! 打印字符串时变量前加&,表明变量是&str切片类型。

// &str 和 &String 不是同样的数据类型:
//      &str 是字符串字面量或切片的引用,它总是不可变的。
//      &String 是 String 类型的引用,它可以是可变的(&mut String)或者不可变的(&String)。
//
// 主要区别在于:
//      &str 引用的字段是常量存储的只读字面量,因此总是不可变的。
//      &String 引用的是堆上动态分配的 String,所以可以通过&mut 来获取可变引用修改其内容。
//
// 而且 &str 和 &String 的内存布局也不同:
//      &str 保存一个指向原始字面量的指针。
//      &String 保存了两个指针,一个指向数据,一个指向容量信息。
//
// 所以在 Rust 中:
//      &str 只能被当作不可变借用,无法修改。
//      &String 可以选择它是否可变,取决于引用类型是 &String 还是 &mut String。
// 总的来说,&str 和 &String 从数据类型到内存表示都有明显差异,不能视为同一种数据。Rust 通过这种设计来保证借用规则和内存安全。

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

    // String 与 &str 转换
    // String 转 &str
    let string1 = format!("{} world!", "hello");
    let str1 = string1.as_str();
    let str2 = &string1;
    // &str 转 String
    let string2 = str1.to_string();
    let string3 = str2.to_string();
    println!("{}", String::from(string1) + &string2 + &string3);

    // let apples = 5;      // 不可变,  let apples = apples + 10;
    // let mut bananas = 5; // 可变, bananas = 20;
    // println!("{}, {}", apples, bananas)

    let mut my_string = "https://example.com/httpend;".to_string();
    // 直接使用 trim_start_matches() 删除开头的 "http"
    my_string = my_string.trim_start_matches("http").to_string();
    // 继续删除结尾的 "end;"
    my_string = my_string.trim_end_matches("end;").to_string();
    println!("Modified string: {}", my_string);

    let number = 3;
    let text = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Other",
    };
    println!("The text is: {}", text);

    // 绿色文本
    println!("\x1b[32mGreen text\x1b[0m");

    // 红色文本
    println!("\x1b[31mRed text\x1b[0m");

    // 黄色文本
    println!("\x1b[33mYellow text\x1b[0m");
}
