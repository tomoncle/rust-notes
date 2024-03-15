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

// 使用 trait objects 的 Box<dyn SimpleConnection> 来存储不同类型的连接，并在运行时根据条件选择具体的连接类型。
//
// 通过这种方式，您可以在 Rust 中实现类似动态加载不同类型连接的功能。
trait SimpleConnection {
    fn establish_connection(&self, url: &str);
}

struct MysqlConnection;

struct PgConnection;

struct SqliteConnection;

impl SimpleConnection for MysqlConnection {
    fn establish_connection(&self, url: &str) {
        println!("Establishing MySQL connection to {}", url);
    }
}

impl SimpleConnection for PgConnection {
    fn establish_connection(&self, url: &str) {
        println!("Establishing PostgreSQL connection to {}", url);
    }
}

impl SimpleConnection for SqliteConnection {
    fn establish_connection(&self, url: &str) {
        println!("Establishing SQLite connection to {}", url);
    }
}

fn main() {
    let connection: Box<dyn SimpleConnection> = if true {
        Box::new(MysqlConnection)
    } else if false {
        Box::new(PgConnection)
    } else {
        Box::new(SqliteConnection)
    };

    connection.establish_connection("some_url_here");
}