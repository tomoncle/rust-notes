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

//! rust 符号学习
//!*  &表示创建引用
//!*  &mut表示创建可变引用
//!*  *表示解引用取值
//!*  &*表示先解引用再引用
fn main() {
    // & 符号： 在Rust中，&符号用于创建引用，它可以用于借用（borrow）变量。
    //        借用是一种在不转移所有权的情况下，允许对变量进行临时访问的机制。
    let x = 5;
    let y = &x; // 创建对x的引用
    println!("x 的值为: {}", x);
    println!("y 的值为: {}", *y); // 解引用y来访问x的值

    // &* 符号： &*符号通常不会在Rust中直接使用，但是可以通过它们来理解引用和解引用的组合。
    //         &*符号在一起使用时，&*表示先解引用再引用。
    let x = 5;
    let y = &x; // 创建对x的引用
    let z = &*y; // 先解引用y，再引用得到z对x的引用
    println!("x 的值为: {}", x);
    println!("z 的值为: {}", *z); // 解引用z来访问x的值

    // * 符号： 在Rust中，*符号用于解引用，它可以用于访问引用指向的值。
    let x = 5;
    let y = &x; // 创建对x的引用
    println!("x 的值为: {}", x);
    println!("y 的值为: {}", *y); // 解引用y来访问x的值

    // &mut 符号： 表示创建可变引用，允许对变量进行可变的借用
    //
    // 声明一个可变变量x，并创建了一个可变引用y来借用x。
    // 然后，我们通过可变引用y修改了x的值，最后输出了修改后的x的值。
    // 这就是&mut符号的使用，它允许我们创建可变引用，从而可以修改被借用的变量的值。
    //
    // Rust不允许:
    //   1.一个变量同时存在可变引用和不可变引用
    //   2.可变引用存在期间,不允许引入额外的不可变引用
    //
    // Rust中可变引用的生命周期问题：
    //   在Rust中，对同一变量的可变引用和不可变引用不能同时存在，因为这可能导致数据竞争和不确定的行为。
    //   当你尝试在使用不可变引用的地方输出x的值时，这与同时持有可变引用y是矛盾的，因为Rust要求在特定作用域内只能有一个可变引用。
    //   确保可变引用y的作用域在输出x的值之前结束。
    //
    let mut x = 5; // 声明一个可变变量x
    let y = &mut x; // 创建对x的可变引用
    *y += 1; // 通过可变引用修改x的值
    println!("y 的值为: {}", y); // 当使用y这个可变引用时，需要在调用x变量之前，确保可变引用y的作用域在输出x的值之前结束
    println!("x 的值为: {}", x); // 输出修改后的x的值

    let str = String::from("hello");
    let ptr = &str as *const String; // 将一个 &str 类型的引用转换为一个指向 String 类型的不可变原生指针(*const String)
    // *const String 类型不能直接使用默认的格式化方式进行输出。
    // 相反，你需要使用 {:?} 或者 {:#?} 这样的格式化方式来输出原生指针。
    // 让我们来修改一下代码，使用 {:p} 格式化方式来输出原生指针的地址：
    println!("ptr 的值为: {:p}", ptr);
    println!("ptr 的值为: {:?}", ptr);
    println!("ptr 的值为: {:#?}", ptr);
    // 获取原生指针 ptr 背后 String 类型的引用，需要注意的是使用 unsafe 块来处理潜在的不安全操作
    let ptr_ref = unsafe { (*ptr).as_str() };
    println!("ptr_ref 的值为: {}", ptr_ref);
}
