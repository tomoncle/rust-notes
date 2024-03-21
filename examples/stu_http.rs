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
use std::path::Path;

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
        let body = &self
            .client
            .get(&self.url)
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
        let text = &self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .expect("错误详情")
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
        let default_os_path =
            std::path::PathBuf::from(dirs::home_dir().unwrap()).join("Workspaces/pki");
        let cert_path = match std::env::consts::OS {
            "windows" => Path::new("E:\\etc\\pki"),
            _ => default_os_path.as_path(),
        };

        // x509 客户端证书
        let client_cert_file = cert_path.join("client.pem");
        let client_cert = std::fs::read_to_string(client_cert_file.to_str().unwrap()).unwrap();
        // x509 客户端证书私钥
        let client_cert_key_file = cert_path.join("client-key.pem");
        let client_cert_key =
            std::fs::read_to_string(client_cert_key_file.to_str().unwrap()).unwrap();

        // 这个是 x509 客户端证书和私钥 内容合并之后的内容
        let client_bundle_cert = format!("{}{}", client_cert, client_cert_key);
        // println!("[{}]",client_bundle_cert);
        /*
            [-----BEGIN CERTIFICATE-----
            MIID3TCCAsWgAwIBAgIUTu0ciuUqNL6UkL7/dSq82tQMwDQYJKoZIhvcNAQEL
            jxyPs2Itf0DbDNHuHrcdXPFWez+Kb60y4tIV/XoJ5q1a
            -----END CERTIFICATE-----
            -----BEGIN RSA PRIVATE KEY-----
            BGHy7OTLSDQBOrBm1XGGHLGWTv3hGTOv+BB9lOBIcXWRkYQmBe3UWjjJf8+SD
            NlQ9YqNKK38C5y6PbrefrWUvbhNJ0mwXEprvwMcGsIx1dE2YnCGH
            -----END RSA PRIVATE KEY-----
            ]
        */
        // 加载 客户端证书和私钥 合并之后的内容，进行加载
        let identity = Identity::from_pem(&client_bundle_cert.as_bytes()).unwrap();

        // CA 证书
        let ca_cert_file = cert_path.join("ca.pem");
        let ca_cert = std::fs::read_to_string(ca_cert_file.to_str().unwrap()).unwrap();
        // 加载 CA 证书
        let ca_certificate = Certificate::from_pem(ca_cert.as_bytes()).unwrap();

        // 构建Client
        let client = reqwest::blocking::Client::builder()
            .use_rustls_tls() // 启用tls配置
            .identity(identity) // 加载客户端证书和私钥
            .add_root_certificate(ca_certificate) // 加载CA证书
            .build()
            .unwrap();

        // 执行 HTTPS 请求
        let response = client
            .get("https://10.18.0.1:6443")
            .send()
            .expect("Send message ERROR")
            .text()
            .unwrap();

        println!("{}", response)
    }
}

// 同步函数测试
fn main() {
    let http_sync = HttpSync::new();
    println!("************ GET *****************");
    http_sync.get();
    println!("************ POST *****************");
    http_sync.post();

    match http_sync.get_result() {
        Ok(s) => println!("请求成功: {}", s),
        Err(e) => println!("错误详情: {}", e.to_string()),
    };

    http_sync.tls();
}
