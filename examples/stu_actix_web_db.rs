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
use std::sync::Once;

use actix_web::{App, get, HttpServer, middleware, Responder, web};
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenv::dotenv;
use log::{debug, error, info};
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
#[derive(Debug, Queryable, Serialize, Insertable)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Queryable, Serialize)]
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
//
// 编译错误：
// = note: LINK : fatal error LNK1181: cannot open input file 'sqlite3.lib'
// linking with `link.exe` failed: exit code: 1181
//
// 解决方法：
// Cargo.toml 添加依赖：libsqlite3-sys = { version = "0.28.0", features = ["bundled"] }
//
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

// #[derive(MultiConnection)] 这将派生出连接到不同 databases.diesel::Connection 的枚举的 implements []和相关特征
//
// 通过将此 derive 应用于此类枚举，您可以在所有内部连接都有效的任何位置将枚举用作连接类型。
// 此 derive 支持包含单个 Tuples 字段的枚举变体。每个元组字段类型都必须实现和许多相关特征。
// 此 databases.diesel::Connection 支持来自 Diesel 本身的连接类型以及第三方连接类型
//
// 实现尝试按照枚举中指定的连接顺序与给定的连接字符串建立新连接。
// 如果一个连接失败，它将尝试下一个连接，依此类推。
// 这意味着，一旦多个连接类型接受某个连接字符串，枚举中的第一个匹配类型将始终建立连接。
// 如果其中一个连接类型是 []，则这一点尤其重要，因为此连接类型接受任意路径。
// 它通常应该作为 enum 中的最后一个条目。
// 如果要控制创建的连接类型，只需手动构造相应的枚举，首先通过内部类型建立连接，
// 然后将结果包装到 enum.diesel::Connection::establishdiesel::SqliteConnection 中
//
// 局限性:
// 派生连接实现只能涵盖 所有内部连接类型。
// 因此，如果一个后端不支持某些 SQL 功能， 例如，与 Returning 子句一样，整个 Connection 实现不会 支持此功能。
// 此外，仅支持一组有限的 SQL 类型：
//
// https://docs.diesel.rs/2.1.x/diesel/derive.MultiConnection.html
// https://docs.diesel.rs/master/diesel_derives/derive.MultiConnection.html
#[derive(diesel::MultiConnection)]
pub enum MultiDBConnection {
    Postgresql(PgConnection),
    Mysql(MysqlConnection),
    Sqlite(SqliteConnection),
}

static INIT: Once = Once::new();
static mut POOL: Option<r2d2::Pool<ConnectionManager<MultiDBConnection>>> = None;

fn db_pool() -> &'static r2d2::Pool<ConnectionManager<MultiDBConnection>> {
    unsafe {
        INIT.call_once(|| {
            info!("initial load connection pool.");
            let database_url = "mysql://root:root@127.0.0.1:3306/db_rust";
            let manager = ConnectionManager::<MultiDBConnection>::new(database_url);
            POOL = Some(
                r2d2::Pool::builder()
                    .max_size(10)
                    .min_idle(Some(5))
                    .build(manager)
                    .unwrap(),
            );
        });
        POOL.as_ref().unwrap()
    }
}

pub fn db_conn() -> Result<PooledConnection<ConnectionManager<MultiDBConnection>>, anyhow::Error> {
    db_pool().get().map_err(|err| {
        error!("获取数据库连接失败: {:?}", err);
        anyhow::Error::from(err)
    })
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
                            .to_string()
                            .replace("\"", "")
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

    // let conn = &mut postgresql_connection();
    // let results = users
    //     .limit(5)
    //     .load::<User>(&mut conn)
    //     .expect("Error loading users");
    debug!("使用单一数据库连接");
    let mut conn = mysql_connection();
    let query = users.limit(5);
    show_sql!(&query);
    let results = query.load::<User>(&mut conn).unwrap();

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

async fn test_pool_datasource(path: web::Path<String>) -> impl Responder {
    use self::t_test::dsl::*;

    debug!("测试数据库连接池");
    // 使用数据库连接池
    let mut conn = db_conn().unwrap();
    // let results = t_test.limit(5).load::<ITest>(conn).unwrap();
    // 拆分上面的查询，用于打印SQL
    let query = t_test.limit(5);
    show_sql!(&query);
    let results = query.load::<ITest>(&mut conn).unwrap();

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

async fn test_singleton_datasource(path: web::Path<String>) -> impl Responder {
    use self::t_test::dsl::*;

    debug!("使用单一数据库连接");
    // 使用单一数据库连接
    let mut conn = mysql_connection();
    // let results = t_test.limit(5).load::<ITest>(conn).unwrap();
    // 拆分上面的查询，用于打印SQL
    let query = t_test.limit(5);
    show_sql!(&query);
    let results = query.load::<ITest>(&mut conn).unwrap();

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

async fn test_multi_datasource(path: web::Path<String>) -> impl Responder {
    use self::t_test::dsl::*;

    debug!("测试多数据源连接");
    // 兼容多种数据库连接驱动
    // https://docs.diesel.rs/2.1.x/diesel/derive.MultiConnection.html
    // https://docs.diesel.rs/master/diesel_derives/derive.MultiConnection.html
    let db_type = env::var("DB_TYPE").unwrap_or_default();
    debug!("当前数据库类型：{}", &db_type);
    let mut conn = match db_type.as_str() {
        "postgres" => MultiDBConnection::Postgresql(postgresql_connection()),
        "mysql" => MultiDBConnection::Mysql(mysql_connection()),
        _ => MultiDBConnection::Sqlite(sqlite_connection()),
    };
    let query = t_test.limit(5);
    show_sql!(&query);
    // 执行通用查询
    let results = query.load::<ITest>(&mut conn).unwrap();
    // 使用 if let 语句来匹配 conn 的具体类型。
    // 如果 conn 是 MultiDBConnection::Postgresql，则会执行 PostgreSQL 特有的查询。
    // ref mut conn 表示对 conn 的可变引用，这样可以在匹配后直接使用它进行查询
    if let MultiDBConnection::Postgresql(ref mut conn) = conn {
        // 在此处执行特定于 PostgreSQL 的查询
        let results = query.load::<ITest>(conn).unwrap();
        debug!("{:?}", results)
    }
    // 如果对多种数据库没有兼容查询方法，可以使用下面方法，每种数据库连接的查询语句都实现一下
    // let results = match &mut conn {
    //     MultiDBConnection::Postgresql(pg_conn) => {
    //         query.load::<ITest>(pg_conn).unwrap()
    //     }
    //     MultiDBConnection::Mysql(mysql_conn) => {
    //         query.load::<ITest>(mysql_conn).unwrap()
    //     }
    //     MultiDBConnection::Sqlite(sqlite_conn) => {
    //         query.load::<ITest>(sqlite_conn).unwrap()
    //     }
    // };

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
    env::set_var("DB_TYPE", "mysql");
    env_logger::init();

    info!("App starting on: {}", "http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/simple/{name}", web::get().to(test_singleton_datasource))
            .route("/multiple/{name}", web::get().to(test_multi_datasource))
            .route("/pool/{name}", web::get().to(test_pool_datasource))
            .service(hello)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
