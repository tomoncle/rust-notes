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
// https://github.com/actix/examples
// https://github.com/robatipoor/rustfulapi

use actix_web::{App, get, HttpServer, Responder, web};
use diesel::prelude::*;
// use diesel::mysql::MysqlConnection;
// use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serde::Serialize;

#[derive(Serialize)]
struct RestHttpResponse<T> {
    code: i32,
    message: String,
    data: T,
}

// 定义数据库表模型
// CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT);
table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
    }
}

// CREATE TABLE t_test (id INTEGER PRIMARY KEY, name TEXT);
table! {
    t_test (id) {
        id -> Integer,
        name -> Varchar,
    }
}

// 定义用户模型
#[derive(Queryable)]
#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Queryable)]
#[derive(Serialize)]
struct ITest {
    id: i32,
    name: String,
}

/// 连接到MySQL数据库 ???
///
/// windows: 编译错误: could not find native static library `mysqlclient`, perhaps an -L flag is missing?
///
/// 1.安装Mysql：For windows install https://downloads.mysql.com/archives/community/
///
/// 2.配置环境变量：export MYSQLCLIENT_LIB_DIR="C:\Program Files\MySQL\MySQL Server 5.7\lib"
///
/// 3.执行命令：cargo install diesel_cli --no-default-features --features mysql
fn establish_connection() -> diesel::mysql::MysqlConnection {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "mysql://root:root@127.0.0.1:3306/db_rust";
    diesel::mysql::MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// 连接 SQLite 数据库
// fn establish_connection() -> diesel::sqlite::SqliteConnection {
//     dotenv().ok();
//     // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let database_url = "sqlite://db_rust.sqlite";
//     diesel::sqlite::SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

// 查询操作示例
async fn index() -> impl Responder {
    use self::users::dsl::*;

    let mut connection = establish_connection();
    let results = users.limit(5)
        .load::<User>(&mut connection)
        .expect("Error loading users");

    let mut array = serde_json::json!([]);
    for user in results {
        let obj = serde_json::json!(&user);
        array.as_array_mut().unwrap().push(obj)
    }
    let res = RestHttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: array,
    };
    // 用serde_json::to_string序列化
    serde_json::to_string(&res).unwrap()
}

async fn test(path: web::Path<String>) -> impl Responder {
    use self::t_test::dsl::*;

    let mut connection = establish_connection();
    let results = t_test.limit(5)
        .load::<ITest>(&mut connection)
        .expect("Error loading t_test");

    let mut array = serde_json::json!([]);
    for test in results {
        let obj = serde_json::json!(&test);
        array.as_array_mut().unwrap().push(obj)
    }
    let res = RestHttpResponse {
        code: 200,
        message: format!("welcome {}!", &path),
        data: array,
    };
    // 用serde_json::to_string序列化
    serde_json::to_string(&res).unwrap()
}

#[get("/hello/{name}")]
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
    let res = RestHttpResponse {
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
            .route("/", web::get().to(index))
            .route("/test/{name}", web::get().to(test))
            .service(hello)
    }).bind("127.0.0.1:8080")?.run().await
}
