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


/// ********************************************
/// * soup 已经不在更新了，代码不再使用；转为 scraper *
/// ********************************************
// use prettytable::{Cell, Row, Table};
// // http 爬虫
// use soup::{NodeExt, QueryBuilderExt, Soup};
//
// fn html_response() -> Result<String, reqwest::Error> {
//     let url = "https://www.downloadkubernetes.com";
//     let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
//     AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
//     let body = reqwest::blocking::Client::new().get(url)
//         .header("User-Agent", user_agent)
//         .send()?
//         .text()?;
//     Ok(body)
// }
//
// fn main() {
//     match html_response() {
//         Ok(s) => {
//             let mut html = s;
//             let doc = Soup::new(html.as_mut_str());
//             let title = doc.tag("title").find().expect("Couldn't find tag 'title'");
//             println!("Title: {}\n", title.text());
//
//             let mut table = Table::new();
//             // 定义一个 数组来 接收每一行需要插入的数据
//             let mut vec = Vec::new();
//
//             // 插入表头
//             for (_i, tag) in doc.tag("th").find_all()
//                 .enumerate()
//                 .filter(|(index, _element)| { *index <= 4 }) {
//                 vec.push(Cell::new(tag.text().as_str()));
//             }
//             table.add_row(Row::new(vec.clone()));
//
//             // 插入行数据
//             for (_i, tr) in doc.tag("tr").find_all()
//                 .enumerate()
//                 .filter(|(index, _element)| { *index > 0 }) {
//                 // 清空数组
//                 vec.clear();
//                 for (_j, td) in tr.tag("td").find_all()
//                     .enumerate()
//                     .filter(|(index, _element)| { *index <= 4 }) {
//                     let mut val = td.text()
//                         .replace("\n", "")
//                         .replace(" ", "");
//                     if val.ends_with("(checksum|signature|certificate)") {
//                         // &*用于解引用val变量,让val不再是一个字符串值的引用,而是一个字符串值本身。
//                         // Rust中解引用的操作符号是&,但val本身就是一个引用,所以需要用&&是不合法的
//                         // 这里使用&*可以“取消”val的引用属性,直接得到底层字符串值
//                         val = String::from("https://") + &*val.replace("(checksum|signature|certificate)", "");
//                     }
//                     vec.push(Cell::new(val.as_str()));
//                 }
//                 table.add_row(Row::new(vec.clone()));
//             }
//             // 打印输出
//             table.printstd();
//         }
//         Err(e) => println!("错误详情: {}", e.to_string()),
//     };
// }

fn html_response() -> String {
    let url = "https://www.downloadkubernetes.com";
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    let body = reqwest::blocking::Client::new().get(url)
        .header("User-Agent", user_agent)
        .send()
        .expect("调用失败")
        .text()
        .unwrap();
    body
}


fn main() {
    use scraper::{Html, Selector};

    let html = html_response();
    let document = Html::parse_document(&html);
    let selector_th = Selector::parse("th").unwrap();
    let selector_td = Selector::parse("td").unwrap();
    for element in document.select(&selector_th) {
        println!("{}", element.text().collect::<String>());
    }

    for (index, element) in document.select(&selector_td).enumerate() {
        println!("{} : {}", index, element.text().collect::<String>());
        if index % 5 == 4 {
            println!("*****************************************************************")
        }
    }
}