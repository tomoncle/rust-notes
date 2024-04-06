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

use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;

// 使用Serde的#[derive(Serialize)]自动生成序列化实现
#[derive(Serialize)]
struct HttpResponse<T> {
    code: i32,
    message: String,
    data: T,
}

#[get("/index")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page."
}

#[get("/hello/{name}")]
async fn hello(path: web::Path<String>) -> impl Responder {
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

async fn users(id: web::Path<String>) -> impl Responder {
    if id.is_empty() {
        return String::from("user list is empty.");
    }
    let data = serde_json::json!({
        "name": format!("Hello World {}!", &id),
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

async fn default(_req: HttpRequest) -> impl Responder {
    "call default method: default data"
}

// 定义路由模块
mod routes {
    use crate::{default, hello, index, users};
    use actix_web::web;

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api") // 配置前缀
                .route("/{name}", web::get().to(default))
                .route("/users/{id}", web::get().to(users))
                .service(index)
                .service(hello),
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("App starting on: {:?}", "127.0.0.1:8080");
    HttpServer::new(|| App::new().configure(routes::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
