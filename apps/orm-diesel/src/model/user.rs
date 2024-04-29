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
use chrono::NaiveDateTime;
use derive_builder::Builder;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::t_user;
use crate::utils::time::{
    get_cst_naive_date_time, naive_date_time_from_option, naive_date_time_from_str,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = t_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id))] // 如果主键不是 id, 需要使用这个属性标记
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub config: String,
    pub state: bool,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_deleted: bool,
    pub delete_time: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = t_user)]
pub struct NewUser {
    pub name: String,
    pub description: Option<String>,
    pub config: String,
    pub state: bool,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
}

// Debug      : 允许使用 {:?} 格式打印结构体的内容。
// AsChangeset: 允许将结构体转换为 Changeset 对象，用于更新数据库记录。
// Default    : 允许使用 Default::default() 创建结构体的默认实例。
// Builder    : 允许使用 Builder 模式创建结构体实例。需要配置外部依赖，可以使用 derive_builder 包
#[derive(Debug, AsChangeset, Default, Builder)]
#[diesel(table_name = t_user)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Option<String>,
    pub state: Option<bool>,
    pub update_time: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
    pub delete_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserView {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
    pub state: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub is_deleted: bool,
    pub delete_time: Option<String>,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        UserView {
            user_id: user.user_id,
            name: user.name,
            description: user.description,
            config: serde_json::from_str(&user.config).unwrap_or(serde_json::Value::Null),
            state: user.state,
            create_time: user.create_time.map(|dt| dt.to_string()),
            update_time: user.update_time.map(|dt| dt.to_string()),
            is_deleted: user.is_deleted,
            delete_time: user.delete_time.map(|dt| dt.to_string()),
        }
    }
}

impl From<&User> for UserView {
    fn from(user: &User) -> Self {
        UserView {
            user_id: user.user_id,
            name: user.name.clone(),
            description: user.description.clone(),
            config: serde_json::from_str(user.config.clone().as_str())
                .unwrap_or(serde_json::Value::Null),
            state: user.state,
            create_time: user.create_time.map(|dt| dt.to_string()),
            update_time: user.update_time.map(|dt| dt.to_string()),
            is_deleted: user.is_deleted,
            delete_time: user.delete_time.map(|dt| dt.to_string()),
        }
    }
}

impl From<UserView> for User {
    fn from(view: UserView) -> Self {
        User {
            user_id: view.user_id,
            name: view.name,
            description: view.description,
            config: view.config.to_string(),
            state: view.state,
            create_time: naive_date_time_from_option(view.create_time),
            update_time: naive_date_time_from_option(view.update_time),
            is_deleted: view.is_deleted,
            delete_time: naive_date_time_from_option(view.delete_time),
        }
    }
}

impl From<&UserView> for User {
    fn from(view: &UserView) -> Self {
        User {
            user_id: view.user_id,
            name: view.name.clone(),
            description: view.description.clone(),
            config: view.config.clone().to_string(),
            state: view.state,
            create_time: view
                .create_time
                .as_ref()
                .and_then(|dt| naive_date_time_from_str(dt)),
            update_time: view
                .update_time
                .as_ref()
                .and_then(|dt| naive_date_time_from_str(dt)),
            is_deleted: view.is_deleted,
            delete_time: view
                .delete_time
                .as_ref()
                .and_then(|dt| naive_date_time_from_str(dt)),
        }
    }
}

impl From<UserBody> for User {
    fn from(body: UserBody) -> Self {
        User {
            name: body.name,
            description: body.description,
            config: body.config.to_string(),
            ..Default::default()
        }
    }
}

impl From<UserBody> for NewUser {
    fn from(body: UserBody) -> Self {
        let now = get_cst_naive_date_time();
        NewUser {
            name: body.name,
            description: body.description,
            config: body.config.to_string(),
            state: true,
            create_time: Some(now),
            update_time: Some(now),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            user_id: 0,
            name: "".to_string(),
            description: None,
            config: "{}".to_string(),
            state: true,
            create_time: Some(get_cst_naive_date_time()),
            update_time: Some(get_cst_naive_date_time()),
            is_deleted: false,
            delete_time: None,
        }
    }
}
