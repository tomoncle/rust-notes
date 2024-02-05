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

fn main() {
    // 用serde_json::json!生成JSON
    //
    // serde_json::json!宏是用于生成JSON字面量的宏
    //      json 是该宏的名称
    //      ! 表明它是一个宏(macro)
    //      (...) 里面是参数传入该宏
    // 该宏会将传入的数据结构编译成一个 JSON 字面量。
    // 举例来说,serde_json::json!({...}) 会编译生成一个等价于 '{"name":"John","age":30}' 字符串字面量。
    // 所以 ! 符号标识它是一个 macro,而不是普通函数,并且需要用括号传入参数
    //

    let data = serde_json::json!({
        "name": format!("Hello World {}!", "macro"),
        "age": 30
    });
    println!("{}", data)

    // 这个代码定义了一个名为json的macro,用于生成JSON字面量。
    //
    // 详细解释:
    //      macro_rules! 定义一个名为macro_rules的宏
    //      json 内部定义了一个名为json的宏
    //      $($json:tt)+ 语法定义了json宏的参数模式
    //      这里$json:tt表示匹配0个或多个任何token
    //      json!(...) => { ... } 块定义了宏的替换体(expansion)
    //      在替换体中调用了名为json_internal!的内部宏
    //      ($($json)+) 将匹配的$json参数直接传给内部宏
    //
    // 所以总体来说:
    //      定义了一个名为json的macro
    //      它以($($json:tt)+) 的模式匹配参数
    //      匹配后调用内部json_internal!宏,直接传递参数
    //
    // 目的就是定义一个外露的json!宏接口:
    //      对用户隐藏内部实现细节
    //      简单地将参数转发给内部json_internal!实现生成JSON
    //      所以它实现了一个外部友好但内部复杂的JSON生成macro接口。
    /*

    #[macro_export(local_inner_macros)]
    macro_rules! json {
        // Hide distracting implementation details from the generated rustdoc.
        ($($json:tt)+) => {
            json_internal!($($json)+)
        };
    }
    */
}