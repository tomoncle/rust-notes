use prettytable::{Cell, Row, Table};
// http 爬虫
use soup::{NodeExt, QueryBuilderExt, Soup};

fn html_response() -> Result<String, reqwest::Error> {
    let url = "https://www.downloadkubernetes.com";
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    let body = reqwest::blocking::Client::new().get(url)
        .header("User-Agent", user_agent)
        .send()?
        .text()?;
    Ok(body)
}

fn main() {
    match html_response() {
        Ok(s) => {
            let mut html = s;
            let doc = Soup::new(html.as_mut_str());
            let title = doc.tag("title").find().expect("Couldn't find tag 'title'");
            println!("Title: {}\n", title.text());

            let mut table = Table::new();
            // 定义一个 数组来 接收每一行需要插入的数据
            let mut vec = Vec::new();

            // 插入表头
            for (_i, tag) in doc.tag("th").find_all()
                .enumerate()
                .filter(|(index, _element)| { *index <= 4 }) {
                vec.push(Cell::new(tag.text().as_str()));
            }
            table.add_row(Row::new(vec.clone()));

            // 插入行数据
            for (_i, tr) in doc.tag("tr").find_all()
                .enumerate()
                .filter(|(index, _element)| { *index > 0 }) {
                // 清空数组
                vec.clear();
                for (_j, td) in tr.tag("td").find_all()
                    .enumerate()
                    .filter(|(index, _element)| { *index <= 4 }) {
                    let mut val = td.text()
                        .replace("\n", "")
                        .replace(" ", "");
                    if val.ends_with("(checksum|signature|certificate)") {
                        // &*用于解引用val变量,让val不再是一个字符串值的引用,而是一个字符串值本身。
                        // Rust中解引用的操作符号是&,但val本身就是一个引用,所以需要用&&是不合法的
                        // 这里使用&*可以“取消”val的引用属性,直接得到底层字符串值
                        val = String::from("https://") + &*val.replace("(checksum|signature|certificate)", "");
                    }
                    vec.push(Cell::new(val.as_str()));
                }
                table.add_row(Row::new(vec.clone()));
            }
            // 打印输出
            table.printstd();
        }
        Err(e) => println!("错误详情: {}", e.to_string()),
    };
}