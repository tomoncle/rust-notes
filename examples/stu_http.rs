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

// rust http 学习
// https://docs.rs/reqwest/latest/reqwest/blocking/struct.Response.html

use reqwest::{Certificate, Identity};

// 定义结构体
struct HttpSync {
    url: String,
    client: reqwest::blocking::Client,
}

// 定义结构体的方法
#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
impl HttpSync {
    // Rust结构体不能给属性定义默认值，只能通过方法进行属性初始化.
    fn new() -> Self {
        HttpSync {
            url: String::from("https://api.tomoncle.com"),
            client: reqwest::blocking::Client::new(),
        }
    }
    /// 因为HttpSync结构体实现了get方法时,使用了self值,这会导致self值被移动,而不能再被后续方法使用。
    ///
    /// 解决方法是不要直接使用self,使用&self引用来获取HttpSync实例:
    fn get(&self) {
        let body = &self.client.get(&self.url)
            .header("token", "123456")
            .query(&[("username", "tom")])
            .send()
            .expect("错误详情")
            .text()
            .unwrap();
        println!("GET Response body: {}\n", body);
    }
    /// 因为HttpSync结构体实现了post方法时,使用了self值,这会导致self值被移动,而不能再被后续方法使用。
    ///
    /// 解决方法是不要直接使用self,使用&self引用来获取HttpSync实例:
    fn post(&self) {
        let body = "{ \"title\": \"Hello\" }";
        let text = &self.client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(body)
            .send().expect("错误详情")
            .text()
            .unwrap();
        println!("POST Response body: {}\n", text);
    }

    /// 测试 ？ 运算符. https://doc.rust-lang.org/std/result/#the-question-mark-operator-
    ///
    /// 因为HttpSync结构体实现了post方法时,使用了self值,这会导致self值被移动,而不能再被后续方法使用。
    ///
    /// 解决方法是不要直接使用self,使用&self引用来获取HttpSync实例:
    fn get_result(&self) -> Result<String, reqwest::Error> {
        use serde_json::Value;
        let response = reqwest::blocking::get(&self.url)?;
        // 定义一个json对象，类型为 serde_json::Value ， 对于复杂json, 使用使用匿名map来解析JSON
        let json: Value = response.json()?;
        println!("测试 ? 运算符     : {:?}\n", json);
        println!("判断 Key 是否存在 : {}\n", !json["message"].is_null());
        println!("获取 message     : {}\n", json["message"]);
        Ok(json.to_string())
    }

    /// 测试 https 证书
    fn tls(&self) {
        let client_cert = std::fs::read_to_string("E:\\etc\\pki\\client.pem").unwrap();
        let client_cert_key = std::fs::read_to_string("E:\\etc\\pki\\client-key.pem").unwrap();
        let ca_cert = std::fs::read_to_string("E:\\etc\\pki\\ca.pem").unwrap();
        let client_bundle_cert = format!("{}{}", client_cert, client_cert_key);
        // println!("[{}]",client_bundle_cert);

        // 构建Client
        // let identity = native_tls::Identity::from_pkcs8(&client_cert.as_bytes(), &client_cert_key.as_bytes()).unwrap();
        // let connector = native_tls::TlsConnector::builder()
        //     .identity(identity)
        //     .add_root_certificate(native_tls::Certificate::from_pem(ca_cert.as_bytes()).unwrap())
        //     .danger_accept_invalid_certs(true)
        //     .build().unwrap();

        let client = reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .identity(Identity::from_pkcs8_pem(&client_cert.as_bytes(), &client_cert_key.as_bytes()).unwrap())
            .add_root_certificate(Certificate::from_pem(ca_cert.as_bytes()).unwrap())
            .build().unwrap();

        // 执行 HTTPS 请求
        let response = client
            .get("https://172.16.71.220:9443")
            .send()
            .expect("Send message ERROR")
            .text().unwrap();

        println!("{}", response)
    }
}

// 同步函数测试
fn main() {
    let http_sync = HttpSync::new();
    println!("************ GET *****************");
    // http_sync.get();
    println!("************ POST *****************");
    // http_sync.post();

    // match http_sync.get_result() {
    //     Ok(s) => println!("请求成功: {}", s),
    //     Err(e) => println!("错误详情: {}", e.to_string()),
    // };

    http_sync.tls();
}

