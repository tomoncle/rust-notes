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
use serde_json::json;
use sysinfo::{System};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let mut json_array = serde_json::json!([]);
    // 接收每一行需要插入的数据
    for (pid, process) in system.processes() {
        // println!("PID : {}", pid);
        // println!("Name: {}", process.name());
        // println!("CMD : {:?}", process.cmd());
        // println!("Exe : {:?}", process.exe());
        // println!("CWD : {:?}", process.cwd());
        json_array.as_array_mut().unwrap().push(json!({
            "pid": pid.as_u32(),
            "name": process.name(),
            "cmd": process.cmd(),
        }));
    }

    let pretty_json = serde_json::to_string_pretty(&json_array).unwrap();
    println!("Json格式化: {}", pretty_json);
}
