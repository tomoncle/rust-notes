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

use actix_web::web;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct RestHttpResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> RestHttpResponse<T> {
    pub fn ok(data: Option<T>) -> RestHttpResponse<T> {
        RestHttpResponse {
            code: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn err(code: i32, message: String) -> RestHttpResponse<T> {
        RestHttpResponse {
            code,
            message,
            data: None,
        }
    }

    pub fn server_err(message: String) -> RestHttpResponse<T> {
        Self::err(500, message)
    }

    pub fn bad_request(message: String) -> RestHttpResponse<T> {
        Self::err(400, message)
    }

    pub fn not_found(message: String) -> RestHttpResponse<T> {
        Self::err(404, message)
    }
}

#[derive(Debug)]
pub struct QueryParser {
    pub query: web::Query<Value>,
}

impl QueryParser {
    pub fn i64(&self, key: &str, default: i64) -> i64 {
        self.query.get(key)
            .and_then(|p| p.as_str())
            .and_then(|p| p.parse::<i64>().ok())
            .unwrap_or(default)
    }
}

// handle_result(User::update(data))
pub fn handle_result<T, E: std::fmt::Display>(result: Result<T, E>) -> web::Json<RestHttpResponse<T>> {
    match result {
        Ok(item) => {
            web::Json(RestHttpResponse::ok(Some(item)))
        }
        Err(error) => web::Json(RestHttpResponse::server_err(error.to_string())),
    }
}
