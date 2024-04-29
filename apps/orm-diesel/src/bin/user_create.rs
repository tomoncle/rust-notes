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

use chrono::{Duration, Utc};
use diesel::{RunQueryDsl, SelectableHelper};

use orm_diesel::model::user::*;

fn get_cst_time() -> chrono::NaiveDateTime {
    let now_utc: chrono::DateTime<Utc> = Utc::now();
    let now_naive: chrono::NaiveDateTime = now_utc.naive_local();
    let now_cst: chrono::NaiveDateTime = now_naive + Duration::hours(8);
    now_cst
}

pub fn create_user() -> User {
    use orm_diesel::schema::t_user;
    use orm_diesel::db::db_conn;
    let new_post = NewUser {
        name: "test".to_string(),
        description: None,
        config: "hello world".to_string(),
        state: true,
        create_time: Some(get_cst_time()),
        update_time: Some(get_cst_time()),
    };

    diesel::insert_into(t_user::table)
        .values(&new_post)
        .returning(User::as_returning())
        .get_result(&mut db_conn())
        .expect("Error saving new post")
}

fn main() {
    let user = create_user();
    let view = UserView::from(&user);
    let value = serde_json::to_string_pretty(&view).unwrap();
    println!("{}", value);
    println!("{:?}", user)
}