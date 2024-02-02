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
        let response = &self.client.get(&self.url)
            .header("token", "123456")
            .query(&[("username", "tom")])
            .send()
            .await.expect("错误详情")
            .text()
            .await.unwrap();
        println!("HttpAsync GET Response body: {}\n", response);
    }

    async fn post(&self) {
        let body = "{ \"title\": \"Hello\" }";
        let text = &self.client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await.expect("错误详情")
            .text()
            .await.unwrap();
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