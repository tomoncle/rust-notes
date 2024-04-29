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

use actix_web::{delete, get, post, put, Responder, web};
use diesel::sql_types::Integer;
use serde_json::Value;


use crate::model::user::*;
use crate::utils::http::RestHttpResponse;

#[get("/user")]
async fn list(query: web::Query<Value>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    log::debug!("list query: {:?}", query);
    let users = User::list();

    let views = users.iter().map(|x| UserView::from(x)).collect::<Vec<_>>();
    let res = RestHttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: views,
    };
    web::Json(res)
}

#[get("/user/{id}")]
async fn get(id: web::Path<i32>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    log::debug!("get path: {:?}", id);
    // let user = User::get(id.to_owned());
    match User::get(id.to_owned()) {
        Some(user) => web::Json(RestHttpResponse {
            code: 200,
            message: "OK".to_string(),
            data: Some(UserView::from(user)),
        }),
        None => web::Json(RestHttpResponse {
            code: 404,
            message: "Not Found".to_string(),
            data: None,
        }),
    }
}

#[post("/user")]
async fn create(body: web::Json<UserBody>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    let data = body.into_inner();
    log::debug!("create body: {:?}", data);
    let user = User::create(data);
    let user_view = UserView::from(user);
    let res = RestHttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: user_view,
    };
    web::Json(res)
}

#[put("/user")]
async fn update(body: web::Json<UserBody>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    let data = body.into_inner();
    if data.id.is_none() {
        return web::Json(RestHttpResponse {
            code: 400,
            message: "id is required".to_string(),
            data: None,
        });
    }
    log::debug!("update body: {:?}", data);
    let user = User::update(data);
    let user_view = UserView::from(user);
    let res = RestHttpResponse {
        code: 200,
        message: "OK".to_string(),
        data: Some(user_view),
    };
    web::Json(res)
}


#[delete("/user/{id}")]
async fn delete(id: web::Path<i32>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    log::debug!("get path: {:?}", id);
    // let user = User::get(id.to_owned());
    let ok = User::delete(id.to_owned());
    if ok {
        web::Json(RestHttpResponse {
            code: 200,
            message: "OK".to_string(),
            data: None::<String>,
        })
    } else {
        web::Json(RestHttpResponse {
            code: 404,
            message: "Not Found".to_string(),
            data: None::<String>,
        })
    }
}