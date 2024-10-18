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

#[allow(dead_code)]
pub mod time;
#[allow(dead_code)]
pub mod http;
pub mod macros;

pub fn show_sql<T: diesel::query_builder::QueryFragment<diesel::pg::Pg>>(query: &T) {
    // println!("\x1b[31mSQL: {}\x1b[0m\n", diesel::debug_query::<diesel::pg::Pg, _>(query).to_string());
    match std::env::var("SHOW_SQL") {
        Ok(bool) => {
            if bool == "true" {
                log::debug!("\x1b[31mSQL ==> {}\x1b[0m",
                    diesel::debug_query::<diesel::pg::Pg, _>(query).to_string().replace("\"", ""));
            }
        }
        Err(_) => {}
    };
}