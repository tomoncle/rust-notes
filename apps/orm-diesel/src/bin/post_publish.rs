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

use diesel::prelude::*;

use orm_diesel::*;
use orm_diesel::db::db_conn;

// cargo run --bin post_publish 1
fn main() {
    use self::schema::t_posts::dsl::*;

    let input_id = match std::env::args().nth(1) {
        Some(val) => val,
        None => {
            eprintln!("publish_post requires a post id");
            return;
        }
    };

    let parsed_id = match input_id.parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid ID");
            return;
        }
    };

    // Now you can use parsed_id as an i32
    println!("Parsed ID: {}", parsed_id);

    let connection = &mut db_conn().unwrap();
    let post = diesel::update(t_posts.find(parsed_id))
        .set(published.eq(true))
        .returning(model::posts::Post::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Published post {}", post.title);
}
