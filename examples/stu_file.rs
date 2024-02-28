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
use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::{DateTime, Local};

fn main() {
    let file_path = "example.txt";

    // 打开文件，如果文件不存在则创建它
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file");

    // 获取更新前的文件内容
    let mut text = String::new();
    file.read_to_string(&mut text).expect("");
    println!("更新前文件内容: \n{}", text);

    // 获取当前时间
    let current_time: DateTime<Local> = Local::now();
    // println!("Current time is: {}", current_time.format("%Y-%m-%d %H:%M:%S"));

    // 向文件写入内容
    let value = format!("{}: Hello, Rust!\n", current_time.format("%Y-%m-%d %H:%M:%S"));
    file.write_all(value.as_bytes()).expect("Unable to write to file");

    // 重新定位文件指针到文件开头
    file.seek(std::io::SeekFrom::Start(0)).expect("Unable to seek");

    // 读取文件内容并打印
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    println!("更新后文件内容: \n{}", contents);
}
