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

// 定义一个打印SQL的宏，外部调用该宏时，使用 use crate::show_sql_info; 引用即可.
//
// 在 Rust 中，#[macro_export] 是一个属性宏，用于定义可以在其他模块中使用的宏。
// 它的作用是将宏导出，使其在当前 crate 之外也能被访问。
#[macro_export]
macro_rules! show_sql_info {
    ($query:expr) => {
        // 这里引用下面两个函数，是为了调用该宏的rs文件中就不用引用这两个函数了，
        // 不然需要在调用该宏的文件中显示的引用，宏中使用的函数
        // use log::debug;
        // use std::env;
        //
        use log::debug;
        use std::env;

        match env::var("SHOW_SQL") {
            Ok(s) => {
                if s == "true" {
                    debug!(
                        "当前执行的SQL: \x1b[31m{:?}\x1b[0m",
                        diesel::debug_query::<diesel::pg::Pg, _>($query)
                            .to_string()
                            .replace("\"", "")
                    );
                }
            }
            Err(_) => {}
        };
    };
}