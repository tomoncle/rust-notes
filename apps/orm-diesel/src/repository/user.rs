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

use std::env;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::{debug, info};

use crate::db::db_conn;
use crate::model::user::*;
use crate::schema::t_user::dsl::*;
use crate::schema::t_user::is_deleted;
use crate::utils::time::get_cst_naive_date_time;

macro_rules! show_sql {
    ($query:expr) => {
        match std::env::var("SHOW_SQL") {
            Ok(s) => {
                if s == "true" {
                    log::debug!(
                        "\x1b[31m{:?}\x1b[0m",
                        diesel::debug_query::<diesel::pg::Pg, _>($query)
                    );
                }
            }
            Err(_) => {}
        };
    };
}
impl User {
    pub fn create(body: UserBody) -> User {
        let dsl = diesel::insert_into(t_user).values(NewUser::from(body));
        show_sql!(&dsl.clone());
        dsl.get_result(&mut db_conn()).expect("创建失败！")
    }

    pub fn update(body: UserBody) -> User {
        let data = UpdateUser {
            name: Some(body.name),
            description: body.description,
            config: Some(body.config.to_string()),
            update_time: Some(get_cst_naive_date_time()),
            ..Default::default()
        };
        let dsl = diesel::update(t_user)
            .filter(user_id.eq(body.id.unwrap()))
            .set(&data);
        show_sql!(&dsl.clone());
        let user: User = dsl.get_result(&mut db_conn()).expect("更新失败！");
        user
    }

    pub fn list() -> Vec<User> {
        t_user
            .filter(is_deleted.eq(false))
            .limit(10)
            .select(User::as_select())
            .load(&mut db_conn())
            .expect("查询列表失败！")
    }

    pub fn get(id: i32) -> Option<User> {
        t_user
            .filter(user_id.eq(id))
            .filter(is_deleted.eq(false))
            .select(User::as_select())
            .first(&mut db_conn())
            .ok()
    }

    pub fn delete(id: i32) -> bool {
        //let dsl = diesel::delete(t_user).filter(user_id.eq(id));
        let dsl = diesel::update(t_user).filter(user_id.eq(id)).set((
            is_deleted.eq(true),
            delete_time.eq(get_cst_naive_date_time()),
        ));
        show_sql!(&dsl.clone());
        dsl.execute(&mut db_conn()).expect("删除失败！") > 0
    }
}
