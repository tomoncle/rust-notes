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

// rust http 异步 学习

// 异步
struct HttpAsync {
    url: String,
    client: reqwest::Client,
}

impl HttpAsync {
    // Rust结构体不能给属性定义默认值，只能通过方法进行属性初始化.
    fn new() -> Self {
        HttpAsync {
            url: String::from("https://api.tomoncle.com"),
            client: reqwest::Client::new(),
        }
    }
    async fn get(&self) {
        let response = &self
            .client
            .get(&self.url)
            .header("token", "123456")
            .query(&[("username", "tom")])
            .send()
            .await
            .expect("错误详情")
            .text()
            .await
            .unwrap();
        println!("HttpAsync GET Response body: {}\n", response);
    }

    async fn post(&self) {
        let body = "{ \"title\": \"Hello\" }";
        let text = &self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("错误详情")
            .text()
            .await
            .unwrap();
        println!("POST Response body: {}\n", text);
    }
}

// 异步函数测试, 需要标记函数为 #[tokio::main]
#[tokio::main]
async fn main() {
    let http_async = HttpAsync::new();
    println!("************ HttpAsync GET *****************");
    http_async.get().await;
    println!("************ HttpAsync POST *****************");
    http_async.post().await;
}
