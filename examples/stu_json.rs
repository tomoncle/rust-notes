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

/// 测试简单的json使用
fn simple_json() {
    // 定义一个json数组， 声明 array 为可变对象的原因是因为您计划往数组中添加元素，这需要对数组进行修改
    let mut array = serde_json::json!([]);
    // 定义一个json对象
    let data = serde_json::json!({
        "name": format!("tom"),
        "age": 30
    });
    // 将对象添加到数组
    // 为什么要使用 data.clone() 是因为您正在将 data 对象添加到 array 中，
    // 而 Rust 中的所有权系统要求在将数据移动到新的位置时，原始数据将不再有效。
    // 因此，在将 data 添加到数组之前，您需要克隆（clone）data 对象，以确保原始数据仍然可用。
    array.as_array_mut().unwrap().push(data.clone());

    // Array [Object {"age": Number(30), "name": String("tom")}]
    println!("rust 数组对象: {:?}", array);
    // [{"age":30,"name":"tom"}]
    println!("json 数组: {}", array.to_string());
    // {"age":30,"name":"tom"}
    println!("rust json对象: {}", data);
    // {"age":30,"name":"tom"}
    println!("json 对象: {}", data.to_string());
}

/// 定义一个json对象, 其中，data使用泛型,
/// #[derive(Serialize)] 表示序列化，struct -> json
#[derive(Serialize)]
struct HttpResponse<T> {
    code: i32,
    message: String,
    data: T,
}

fn object_to_json() {
    let res1 = HttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: "success!",
    };
    // 用serde_json::to_string序列化
    println!("测试res1：{}", serde_json::to_string(&res1).unwrap());

    // 用serde_json::json!生成JSON
    //
    // serde_json::json!宏是用于生成JSON字面量的宏
    //      json 是该宏的名称
    //      ! 表明它是一个宏(macro)
    //      (...) 里面是参数传入该宏
    // 该宏会将传入的数据结构编译成一个 JSON 字面量。
    // 举例来说,serde_json::json!({...}) 会编译生成一个等价于 '{"name":"John","age":30}' 字符串字面量。
    // 所以 ! 符号标识它是一个 macro,而不是普通函数,并且需要用括号传入参数
    let res2 = HttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: serde_json::json!({"name": format!("tom"),"age": 30}),
    };
    // 用serde_json::to_string 序列化
    println!("测试res2：{}", serde_json::to_string(&res2).unwrap());
}

/// json 解析
fn json_parser() {
    let json_str = r#"
        {
            "name": "Alice",
            "age": 30,
            "is_student": false
        }
    "#;
    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();

    println!("Name: {}", parsed["name"]);
    println!("Age: {}", parsed["age"]);
    println!("Is Student: {}", parsed["is_student"]);
}

/// 定义一个User对象， 测试反序列化
/// #[derive(Deserialize)] 表示反序列化，json -> struct
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: i32,
    is_student: bool,
}

/// 反序列化 json 字符串为 struct 对象
fn json_to_struct() {
    let json_str = r#"
        {
            "name": "Alice",
            "age": 30,
            "is_student": false
        }
    "#;
    // serde_json::from_str 反序列化
    let user: User = serde_json::from_str(json_str).unwrap();
    println!("User: {:?}", user);
}


/// json 组装
fn json_assemble() {
    // 创建一个空的 JSON 对象
    let mut json_obj = serde_json::json!({});

    // 添加键值对到 JSON 对象
    json_obj["name"] = serde_json::json!("Alice");
    json_obj["age"] = serde_json::json!(30);
    json_obj["is_student"] = serde_json::json!(false);

    // 打印 JSON 对象
    // {"age":30,"is_student":false,"name":"Alice"}
    println!("Json: {}", json_obj);

    // 使用 serde_json::to_string_pretty 格式化输出 Json
    let pretty_json = serde_json::to_string_pretty(&json_obj).unwrap();
    println!("Json格式化: {}", pretty_json);


    // 创建一个空的 Json 数组
    let mut json_array = serde_json::json!([]);
    // 添加 JSON 对象到数组
    json_array.as_array_mut().unwrap().push(serde_json::json!({"name":"github"}));
    json_array.as_array_mut().unwrap().push(serde_json::json!({"name":"google"}));
    // 打印 json 数组
    // [{"name":"github"},{"name":"google"}]
    println!("json数组: {}", json_array);
}

fn main() {
    // simple_json();
    // object_to_json();
    // json_parser();
    // json_to_struct();
    json_assemble();
}