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
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::posts;
use crate::schema::t_blueprint;

/*
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
*/
#[derive(Queryable, Selectable)]
// Queryable 将生成从 SQL 查询加载Post结构所需的所有代码;
// Selectable 根据模型类型构造匹配的 select 子句
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = t_blueprint)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blueprint {
    pub blueprint_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
    pub config: String,
    pub state: bool,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub is_deleted: bool,
    pub delete_time: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = t_blueprint)]
pub struct NewBlueprint<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub user_id: &'a str,
    pub config: &'a str,
    pub create_time: &'a NaiveDateTime,
    pub update_time: &'a NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintView {
    pub blueprint_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
    pub config: String,
    pub state: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub is_deleted: bool,
    pub delete_time: Option<String>,
}


impl From<Blueprint> for BlueprintView {
    fn from(blueprint: Blueprint) -> Self {
        BlueprintView {
            blueprint_id: blueprint.blueprint_id,
            name: blueprint.name,
            description: blueprint.description,
            user_id: blueprint.user_id,
            config: blueprint.config,
            state: blueprint.state,
            create_time: blueprint.create_time.map(|dt| dt.to_string()),
            update_time: blueprint.update_time.map(|dt| dt.to_string()),
            is_deleted: blueprint.is_deleted,
            delete_time: blueprint.delete_time.map(|dt| dt.to_string()),
        }
    }
}

impl From<&Blueprint> for BlueprintView {
    fn from(blueprint: &Blueprint) -> Self {
        BlueprintView {
            blueprint_id: blueprint.blueprint_id,
            name: blueprint.name.clone(),
            description: blueprint.description.clone(),
            user_id: blueprint.user_id.clone(),
            config: blueprint.config.clone(),
            state: blueprint.state,
            create_time: blueprint.create_time.map(|dt| dt.to_string()),
            update_time: blueprint.update_time.map(|dt| dt.to_string()),
            is_deleted: blueprint.is_deleted,
            delete_time: blueprint.delete_time.map(|dt| dt.to_string()),
        }
    }
}

impl From<BlueprintView> for Blueprint {
    fn from(view: BlueprintView) -> Self {
        Blueprint {
            blueprint_id: view.blueprint_id,
            name: view.name,
            description: view.description,
            user_id: view.user_id,
            config: view.config,
            state: view.state,
            create_time: view.create_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
            update_time: view.update_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
            is_deleted: view.is_deleted,
            delete_time: view.delete_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
        }
    }
}

impl From<&BlueprintView> for Blueprint {
   fn from(view: &BlueprintView) -> Self {
        Blueprint {
            blueprint_id: view.blueprint_id,
            name: view.name.clone(),
            description: view.description.clone(),
            user_id: view.user_id.clone(),
            config: view.config.clone(),
            state: view.state,
            create_time: view.create_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
            update_time: view.update_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
            is_deleted: view.is_deleted,
            delete_time: view.delete_time.as_ref().and_then(|dt| NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S").ok()),
        }
    }
}
