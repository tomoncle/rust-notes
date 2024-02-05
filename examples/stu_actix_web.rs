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

use actix_web::{App, get, HttpRequest, HttpServer, Responder, web};
use serde::Serialize;

// 使用Serde的#[derive(Serialize)]自动生成序列化实现
#[derive(Serialize)]
struct HttpResponse<T> {
    code: i32,
    message: String,
    data: T,
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page."
}

async fn hello(path: web::Path<String>) -> impl Responder {
    // 用serde_json::json!生成JSON
    //
    // serde_json::json!宏是用于生成JSON字面量的宏
    //      json 是该宏的名称
    //      ! 表明它是一个宏(macro)
    //      (...) 里面是参数传入该宏
    // 该宏会将传入的数据结构编译成一个 JSON 字面量。
    // 举例来说,serde_json::json!({...}) 会编译生成一个等价于 '{"name":"John","age":30}' 字符串字面量。
    // 所以 ! 符号标识它是一个 macro,而不是普通函数,并且需要用括号传入参数
    let data = serde_json::json!({
        "name": format!("Hello World {}!", &path),
        "age": 30
    });
    let res = HttpResponse {
        code: 200,
        message: "OK".to_string(),
        data,
    };
    // 用serde_json::to_string序列化
    serde_json::to_string(&res).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App starting on: {:?}", "127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/{name}", web::get().to(hello))
    }).bind(("127.0.0.1", 8080))?.run().await
}
