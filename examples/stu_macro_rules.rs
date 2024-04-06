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
/// ############################## 声明宏 ##############################
/// 声明式宏（Declarative Macros）：声明式宏也称为 macro_rules! 宏，是 Rust 中最常见的宏类型之一。
/// 通过 macro_rules! 宏，你可以定义模式匹配规则，用于将输入的代码模式转换成输出的代码模式。

// 声明宏：该宏的作用是通过调用println!宏打印出一个问候语。
macro_rules! greet {
    ($name:expr) => {
        // 打印问候语
        println!("Hello, {}!", $name)
    };
}

// 声明宏：
// 它接受任意数量的参数，并通过println!宏将这些参数以调试格式输出。
// 具体实现中，($($arg:tt)*)表示接受任意类型的token树作为参数，
// 并在println!中使用{:#?}格式化输出
macro_rules! print_args {
    ($($arg:tt)*) => {
        // 打印输入参数
        println!("args: {:#?}", ($($arg)*))
    };
}

fn say_hello(a: &str, b: &str) {
    let value = a.to_string() + " " + b;
    greet!(value);
    print_args!(a, b);
}

fn main() {
    say_hello("java", "python")
}
