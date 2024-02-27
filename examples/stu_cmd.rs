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
use clap::Parser;

#[derive(Parser)]
#[command(name = "test")]
#[command(author = "tomoncle")]
#[command(version = "1.0")]
#[command(about = "a tutorial of crate clap for rust cmd demo.", long_about = None)]
struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// 需要转换的系统目录（绝对路径）
    #[arg(short, long)]
    input: Option<String>,

    /// 转换文件保存的目录（绝对路径）
    #[arg(short, long, default_value = "./")]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input.unwrap_or_else(|| "hello".to_owned());
    let output = cli.output.unwrap_or_else(|| "world".to_owned());
    println!("{input} {output}");
}