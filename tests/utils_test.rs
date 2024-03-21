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

// 参考链接：https://course.rs/test/unit-integration-test.html
//
// 目前来说，Rust 只支持对 lib 类型的包进行集成测试，对于二进制包例如 src/main.rs 是无能为力的。
// 原因在于，我们无法在其它包中使用 use 引入二进制包，而只有 lib 类型的包才能被引入，例如 src/lib.rs。
// 所以想要在 src/tests 中进行测试的包或方法，需要在 src/lib.rs 中引用，示例如下：
//    pub mod utils;
//    pub mod enums;

// rust_notes 为当前项目名(rust-notes 自动转为 rust_notes)
use rust_notes::utils::country::{china, usa};
use rust_notes::utils::fake_structs::Person;
use rust_notes::utils::json::JsonConverter;

#[test]
fn convert_json() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let json_str = JsonConverter::convert_json(&person);
    assert_eq!(false, json_str.is_empty());
}

#[test]
fn convert_object() {
    let json_str = r#"
        {
            "name": "Alice",
            "age": 30
        }
    "#;
    let json_obj: Person = JsonConverter::convert_object(&json_str);
    assert_eq!(30, json_obj.age)
}

#[test]
fn convert_json_array() {
    let persons = vec![
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];
    let json_array_str = JsonConverter::convert_json_array(&persons);
    assert_eq!(false, json_array_str.is_empty());
}

#[test]
fn enums() {
    assert_eq!("USA", usa::en());
    assert_eq!("中国", china::zh());
}
