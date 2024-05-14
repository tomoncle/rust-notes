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

use std::env;

use actix_web::{get, middleware, web, App, HttpServer, Responder};
use diesel::debug_query;
use diesel::prelude::*;
use dotenv::dotenv;
use log::{debug, info};
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
#[derive(Queryable, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Queryable, Serialize)]
struct ITest {
    id: i32,
    name: String,
}

/*
连接到MySQL数据库 ???
windows: 编译错误: could not find native static library `mysqlclient`, perhaps an -L flag is missing?

1.安装Mysql：For windows install https://downloads.mysql.com/archives/community/
2.配置环境变量：export MYSQLCLIENT_LIB_DIR="C:\Program Files\MySQL\MySQL Server 5.7\lib"
3.执行命令：cargo install diesel_cli --no-default-features --features mysql
4.执行命令：cargo clean

macOS: 编译错误
1.安装Mysql：brew install mysql-client
2.配置环境变量:
    ## mysql config
    # If you need to have mysql-client first in your PATH, run:
    export PATH="/opt/homebrew/opt/mysql-client/bin:$PATH"

    # For compilers to find mysql-client you may need to set:
    export LDFLAGS="-L/opt/homebrew/opt/mysql-client/lib"
    export CPPFLAGS="-I/opt/homebrew/opt/mysql-client/include"
3.执行命令：cargo install diesel_cli --no-default-features --features mysql
4.执行命令：cargo clean
*/
fn mysql_connection() -> MysqlConnection {
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "mysql://root:root@127.0.0.1:3306/db_rust";
    let err_msg = &format!("Error connecting to {}", database_url);
    MysqlConnection::establish(&database_url).expect(err_msg)
}

// 连接 SQLite 数据库
fn sqlite_connection() -> SqliteConnection {
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "sqlite://db_rust.sqlite";
    let err_msg = &format!("Error connecting to {}", database_url);
    SqliteConnection::establish(&database_url).expect(err_msg)
}

/*
连接到postgresql数据库 ???
windows: 编译错误: LINK : fatal error LNK1181: 无法打开输入文件“libpq.lib”

1.安装postgresql: For windows install https://www.postgresql.org/download/windows/
2.配置环境变量: export PQ_LIB_DIR="C:\Program Files\Postgresql-16.2-1\pgsql\lib"
3.执行命令: cargo install diesel_cli --no-default-features --features postgres
4.执行命令: cargo clean

macOS: 编译错误
1.安装postgresql: brew install libpq
2.配置环境变量:
    ## Postgresql
    export PATH="/opt/homebrew/opt/libpq/bin:$PATH"
    export LDFLAGS="-L/opt/homebrew/opt/libpq/lib:$LDFLAGS"
    export CPPFLAGS="-I/opt/homebrew/opt/libpq/include:$CPPFLAGS"
    ## rust postgresql
    export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/opt/libpq/lib"
3.执行命令: cargo install diesel_cli --no-default-features --features postgres
4.执行命令: cargo clean

*/
// 连接 postgresql 数据库
fn postgresql_connection() -> PgConnection {
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "postgres://postgres:postgres@172.16.61.135:5432/db_rust";
    let err_msg = &format!("Error connecting to {}", database_url);
    PgConnection::establish(&database_url).expect(err_msg)
}

// 定义宏来打印 SQL 查询语句
macro_rules! show_sql {
    ($query:expr) => {
        // let sql_query = debug_query::<diesel::pg::Pg, _>(&query);
        // debug!("{:?}", sql_query);
        match env::var("SHOW_SQL") {
            Ok(s) => {
                if s == "true" {
                    debug!(
                        "\x1b[31m{:?}\x1b[0m",
                        diesel::debug_query::<diesel::pg::Pg, _>($query)
                    );
                }
            }
            Err(_) => {}
        };
    };
}

// 查询操作示例
async fn index() -> impl Responder {
    use self::users::dsl::*;
    let conn = &mut postgresql_connection();
    // let results = users
    //     .limit(5)
    //     .load::<User>(&mut conn)
    //     .expect("Error loading users");
    let query = users.limit(5);
    show_sql!(&query);
    let results = query.load::<User>(conn).unwrap();
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

    let conn = &mut postgresql_connection();
    let results = t_test.limit(5).load::<ITest>(conn).unwrap();

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
    let data = serde_json::json!({
        "name": format!("Hello World {:?}!", &path),
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
    env::set_var("RUST_LOG", "debug");
    env::set_var("SHOW_SQL", "true");
    env_logger::init();

    info!("App starting on: {}", "http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/test/{name}", web::get().to(test))
            .service(hello)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
