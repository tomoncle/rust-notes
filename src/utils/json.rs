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

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
在 Rust 中，pub(crate) 和 pub 是用来修饰结构体、枚举、函数等项的访问权限修饰符。
它们之间的区别在于可见性范围不同：

    pub(crate) struct JsonConverter; : 使用 pub(crate) 表示该项对当前 crate 内可见，
    即只有在当前 crate（包含当前模块的 crate）内部可以访问该项，其他 crate 无法访问。
    这样的可见性范围是最小的，仅限于当前 crate。

    pub struct JsonConverter;        : 使用 pub 表示该项对外部 crate 可见，即其他 crate 可以访问该项。
    这种可见性范围是更广泛的，任何其他 crate 都可以使用该项。

因此，pub(crate) 更多地用于限制项的可见性范围，只允许在当前 crate 内部使用，而 pub 则是用于将项暴露给外部 crate 使用。
根据项目的需求和设计，您可以根据实际情况选择适合的可见性修饰符来控制项的访问范围。
 */

pub struct JsonConverter;

impl JsonConverter {
    pub fn convert_json<T: Serialize>(data: &T) -> String {
        serde_json::to_string(data).unwrap()
    }

    pub fn convert_object<T: for<'a> Deserialize<'a>>(json_str: &str) -> T {
        serde_json::from_str(json_str).unwrap()
    }

    pub fn convert_json_array<T: Serialize>(data: &Vec<T>) -> String {
        let json_array: Vec<Value> = data.iter().map(|item| json!(item)).collect();
        serde_json::to_string(&json_array).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::fake_structs::Person;

    use super::*;

    #[test]
    fn convert_json_test() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let json_str = JsonConverter::convert_json(&person);
        assert_eq!(json_str.is_empty(), false);
    }

    #[test]
    fn convert_json_array_test() {
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
        assert_eq!(json_array_str.is_empty(), false);
    }

    #[test]
    fn convert_object() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let json_str = JsonConverter::convert_json(&person);
        let json_obj: Person = JsonConverter::convert_object(&json_str);
        assert_eq!(json_obj.age, 30);
    }
}
