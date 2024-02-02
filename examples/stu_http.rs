// rust http 学习
// https://docs.rs/reqwest/latest/reqwest/blocking/struct.Response.html

// 定义结构体
struct HttpSync {
    url: String,
    client: reqwest::blocking::Client,
}

// 定义结构体的方法
impl HttpSync {
    // Rust结构体不能给属性定义默认值，只能通过方法进行属性初始化.
    fn new() -> Self {
        HttpSync {
            url: String::from("https://api.tomoncle.com"),
            client: reqwest::blocking::Client::new(),
        }
    }
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

    // 测试 ？ 运算符. https://doc.rust-lang.org/std/result/#the-question-mark-operator-
    fn get_qm(&self) -> Result<String, reqwest::Error> {
        use serde_json::Value;
        let response = reqwest::blocking::get(&self.url)?;
        // 定义一个json对象，类型为 serde_json::Value ， 对于复杂json, 使用使用匿名map来解析JSON
        let json: Value = response.json()?;
        println!("测试 ? 运算符     : {:?}\n", json);
        println!("判断 Key 是否存在 : {}\n", !json["message"].is_null());
        println!("获取 message     : {}\n", json["message"]);
        Ok(json.to_string())
    }
}

// 同步函数测试
fn main() {
    let http_sync = HttpSync::new();
    println!("************ GET *****************");
    http_sync.get();
    println!("************ POST *****************");
    http_sync.post();

    match http_sync.get_qm() {
        Ok(s) => println!("请求成功: {}", s),
        Err(e) => println!("错误详情: {}", e.to_string()),
    };
}

