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


use anyhow::Context;
use diesel::{Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;

use crate::db::db_conn;
use crate::model::user::*;
use crate::schema::t_user::dsl::*;
use crate::utils::show_sql;
use crate::utils::time::get_cst_naive_date_time;

// 对于可能出现异常地调用，建议使用 Result<User, anyhow::Error> 返回值进行返回，
// 异常使用 ? 往上层调用方抛出。上层进行异常处理
//
// 这里使用 map_err(|err| {...}) 是为了方便记录日志输出
#[allow(dead_code)]
impl User {
    pub fn create(body: UserBody) -> Result<User, anyhow::Error> {
        let mut conn = db_conn()?;
        conn.transaction::<_, anyhow::Error, _>(|conn| {
            let dsl = diesel::insert_into(t_user).values(NewUser::from(body));
            show_sql(&dsl.clone());
            dsl.get_result(conn).map_err(|err| {
                error!("创建用户失败: {:?}", err);
                anyhow::Error::from(err)
            })
        })
    }

    pub fn update(body: UserBody) -> Result<User, anyhow::Error> {
        let mut conn = db_conn()?;
        conn.transaction::<_, anyhow::Error, _>(|conn| {
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
            show_sql(&dsl.clone());
            dsl.get_result(conn).map_err(|err| {
                error!("更新用户失败: {:?}", err);
                anyhow::Error::from(err)
            })
        })
    }

    pub fn list(mut page: i64, mut page_size: i64) -> Result<Vec<User>, anyhow::Error> {
        let mut conn = db_conn()?;
        page = page.max(1);
        page_size = page_size.max(0).min(100);
        let dsl = t_user.filter(is_deleted.eq(false))
            .order((name.desc(), create_time.desc()))
            .offset((page - 1) * page_size)
            .limit(page_size)
            .select(User::as_select());

        show_sql(&dsl.clone());
        dsl.load(&mut conn)
            .map_err(|err| {
                error!("获取用户列表失败: {:?}", err);
                anyhow::Error::from(err)
            })
    }

    pub fn get(id: i32) -> Result<Option<User>, anyhow::Error> {
        let mut conn = db_conn().with_context(|| "无法获取数据库连接")?;
        t_user
            .filter(user_id.eq(id))
            .filter(is_deleted.eq(false))
            .select(User::as_select())
            .first(&mut conn)
            .optional()// 处理可能没有找到记录的情况,并将结果包装在 Option 中返回
            // .with_context(|| format!("无法从数据库中获取 ID 为 {} 的用户", id))
            .map_err(|err| {
                error!("{}",format!("无法从数据库中获取 ID 为 {} 的用户: {:?}", id, err));
                anyhow::Error::from(err)
            })
    }

    pub fn delete(id: i32) -> Result<usize, anyhow::Error> {
        let mut conn = db_conn()?;
        conn.transaction(|conn| {
            let dsl = diesel::update(t_user)
                .filter(user_id.eq(id))
                .set((is_deleted.eq(true), delete_time.eq(get_cst_naive_date_time()), ));
            show_sql(&dsl.clone());
            dsl.execute(conn).map_err(|err| {
                error!("删除用户失败: {:?}", err);
                anyhow::Error::from(err)
            })
        })
    }

    fn unsafe_create(body: UserBody) -> User {
        let dsl = diesel::insert_into(t_user).values(NewUser::from(body));
        show_sql(&dsl.clone());
        dsl.get_result(&mut db_conn().unwrap()).expect("创建失败！")
    }

    fn unsafe_update(body: UserBody) -> User {
        let binding = UpdateUser {
            name: Some(body.name),
            description: body.description,
            config: Some(body.config.to_string()),
            update_time: Some(get_cst_naive_date_time()),
            state: Some(true),
            is_deleted: Some(false),
            delete_time: None,
        };
        let dsl = diesel::update(t_user)
            .filter(user_id.eq(body.id.unwrap()))
            .set(&binding);
        show_sql(&dsl.clone());
        let user: User = dsl.get_result(&mut db_conn().unwrap()).expect("更新失败！");
        user
    }

    fn unsafe_list() -> Vec<User> {
        t_user
            .filter(is_deleted.eq(false))
            .limit(10)
            .select(User::as_select())
            .load(&mut db_conn().unwrap())
            .expect("查询列表失败！")
    }

    fn unsafe_get(id: i32) -> Option<User> {
        t_user
            .filter(user_id.eq(id))
            .filter(is_deleted.eq(false))
            .select(User::as_select())
            .first(&mut db_conn().unwrap())
            .ok()
    }

    fn unsafe_delete(id: i32) -> bool {
        //let dsl = diesel::delete(t_user).filter(user_id.eq(id));
        let dsl = diesel::update(t_user).filter(user_id.eq(id)).set((
            is_deleted.eq(true),
            delete_time.eq(get_cst_naive_date_time()),
        ));
        show_sql(&dsl.clone());
        dsl.execute(&mut db_conn().unwrap()).expect("删除失败！") > 0
    }
}

#[allow(dead_code)]
impl UserQueryBuilder {
    fn new() -> Self {
        UserQueryBuilder {
            query: t_user.filter(is_deleted.eq(false)).into_boxed(),
        }
    }

    fn order_by(mut self, order_by: &[(String, bool)]) -> Self {
        for (field, is_asc) in order_by {
            match field.as_str() {
                "name" => {
                    if *is_asc {
                        self.query = self.query.then_order_by(name.asc());
                    } else {
                        self.query = self.query.then_order_by(name.desc());
                    }
                }
                "create_time" => {
                    if *is_asc {
                        self.query = self.query.then_order_by(create_time.asc());
                    } else {
                        self.query = self.query.then_order_by(create_time.desc());
                    }
                }
                // Add more fields as needed
                // ...
                _ => {// Handle unknown fields or do nothing
                    println!("Warn: 不支持的排序参数：{}", field)
                }
            }
        }
        self
    }

    fn offset(mut self, offset: i64) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    fn limit(mut self, limit: i64) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    fn load(self) -> Result<Vec<User>, diesel::result::Error> {
        let conn = &mut db_conn().unwrap();
        show_sql(&self.query);
        self.query.select(User::as_select()).load(conn)
    }
}


#[cfg(test)]
mod tests {
    use crate::repository::user::UserQueryBuilder;

    #[test]
    #[cfg(feature = "local_runtime")]
    fn test_user_order_by() {
        let order_by = vec![
            ("name".to_string(), true),
            ("create_time".to_string(), false),
            ("foo".to_string(), true),
        ];
        let users = UserQueryBuilder::new()
            .order_by(&order_by)
            .offset(0)
            .limit(10)
            .load()
            .unwrap();
        println!("{:?}", users)
    }
}
