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
use log::debug;
use serde_json::Value;

use crate::model::user::*;
use crate::utils::http::{QueryParser, RestHttpResponse};

#[get("/user")]
async fn list(query: web::Query<Value>) -> impl Responder {
    debug!("list query: {:?}", query);
    let params = QueryParser { query };
    let page = params.i64("page", 1);
    let page_size = params.i64("page_size", 10);

    match User::list(page, page_size) {
        Ok(users) => {
            let views = users.iter().map(|x| UserView::from(x)).collect::<Vec<_>>();
            web::Json(RestHttpResponse::ok(Some(views)))
        }
        Err(error) => web::Json(RestHttpResponse::server_err(error.to_string()))
    }
}

#[get("/user/{id}")]
async fn get(id: web::Path<i32>) -> impl Responder {
    let user_id = id.into_inner();
    debug!("get path: {:?}", user_id);
    match User::get(user_id) {
        Ok(Some(user)) => {
            web::Json(RestHttpResponse::ok(Some(UserView::from(user))))
        }
        Ok(None) => {
            web::Json(RestHttpResponse::not_found(format!("user [{:?}] not found!", user_id)))
        }
        Err(err) => {
            web::Json(RestHttpResponse::server_err(err.to_string()))
        }
    }
}

#[post("/user")]
async fn create(body: web::Json<UserBody>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    let data = body.into_inner();
    debug!("create body: {:?}", data);
    match User::create(data) {
        Ok(user) => web::Json(RestHttpResponse::ok(Some(UserView::from(user)))),
        Err(error) => web::Json(RestHttpResponse::server_err(error.to_string()))
    }
}

#[put("/user")]
async fn update(body: web::Json<UserBody>) -> impl Responder {
    // let data: Value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    let data = body.into_inner();
    debug!("update body: {:?}", data);
    if data.id.is_none() {
        return web::Json(RestHttpResponse::bad_request("id is required".to_string()));
    }
    match User::update(data) {
        Ok(user) => web::Json(RestHttpResponse::ok(Some(UserView::from(user)))),
        Err(error) => web::Json(RestHttpResponse::server_err(error.to_string()))
    }
}

#[delete("/user/{id}")]
async fn delete(id: web::Path<i32>) -> impl Responder {
    debug!("get path: {:?}", id);
    match User::delete(id.into_inner()) {
        Ok(line) => {
            if line > 0 {
                web::Json(RestHttpResponse::<String>::ok(None))
            } else {
                web::Json(RestHttpResponse::<String>::server_err("删除失败!".to_string()))
            }
        }
        Err(error) => web::Json(RestHttpResponse::server_err(error.to_string()))
    }
}