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

use std::fs;
use std::path::Path;

fn main() {
    println!("当前执行目录：{}", std::env::current_dir().unwrap().display());
    println!("当前执行目录：{}", Path::new("./").display());
    /*
      测试目录
    */
    let input_dir = "E:\\下载\\阿里云盘";
    // 转为Path
    let input_dir_path = Path::new(input_dir);
    // 获取文件名称(最后一层)
    let input_dir_name = input_dir_path.file_name().unwrap();
    // unwrap_or_else在parent为空时返回当前目录"."
    let input_parent_dir = input_dir_path.parent()
        .unwrap_or_else(|| Path::new("."));
    // 获取 input_dir 的父级目录：E:\下载, 目录名称：阿里云盘
    println!("获取 input_dir 的父级目录：{}, 目录名称：{}",
             input_parent_dir.display(),
             input_dir_name.to_str().unwrap());

    // 测试目录拼接
    let output_dir = "D:\\test";
    let output_dir_path = Path::new(output_dir);
    let output_dir_new_path = output_dir_path.join(input_dir_name.to_str().unwrap().to_string());

    // 获取 output_dir 绝对路径：D:\test\阿里云盘
    println!("获取 output_dir 绝对路径：{}", output_dir_new_path.display());

    // 测试目录的根路径
    println!("目录的根路径：{}", input_dir_path.ancestors().last().unwrap_or(input_dir_path).to_path_buf().display());

    // 判断文件夹是否为空
    println!("判断文件夹是否为空：{}", fs::read_dir("E:\\backup")
        .map_or(0, |entries| entries
            .filter_map(|entry| entry.ok()).count()) == 0)
}