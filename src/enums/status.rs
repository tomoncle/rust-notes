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

// 枚举
#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
#[derive(Debug)]
pub(crate) enum Status {
    Start,
    Running,
    Success,
    Fail,
    Stop,
}

#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
impl Status {
    fn to_string(self) -> &'static str {
        match self {
            Status::Start => "Start",
            Status::Running => "Running",
            Status::Success => "Success",
            Status::Fail => "Fail",
            Status::Stop => "Stop",
        }
    }
}

#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
#[derive(Debug)]
pub(crate) enum State {
    OK,
    ERR,
}

#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
impl State {
    fn to_string(self) -> &'static str {
        match self {
            State::OK => "OK",
            State::ERR => "ERR",
        }
    }
}

// 单元测试的惯例是将测试代码的模块跟待测试的正常代码放入同一个文件中，
// Rust 是支持对私有函数进行测试的。
//
//
// 条件编译 #[cfg(test)] :
//
// 上面代码中的 #[cfg(test)] 标注可以告诉 Rust 只有在 cargo test 时才编译和运行模块 tests，其它时候这段代码不会执行，
// 例如在 cargo build 时。这么做有几个好处：
//
//         1.节省构建代码时的编译时间
//         2.减小编译出的可执行文件的体积
//
//
// 其实集成测试就不需要这个标注，因为它们被放入单独的目录文件中（src/tests/*），而单元测试是跟正常的逻辑代码在同一个文件，
// 因此必须对其进行特殊的标注，以便 Rust 可以识别。
//
//
// 在 #[cfg(test)] 中，cfg 是配置 configuration 的缩写，它告诉 Rust ：
//
//         当 test 配置项存在时，才运行下面的代码，而 cargo test 在运行时，
//         就会将 test 这个配置项传入进来，因此后面的 tests 模块会被包含进来。
// 这是典型的条件编译，Cargo 会根据指定的配置来选择是否编译指定的代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_test() {
        assert_eq!("Start", Status::Start.to_string());
        assert_eq!("Running", Status::Running.to_string());
        assert_eq!("Success", Status::Success.to_string());
        assert_eq!("Fail", Status::Fail.to_string());
        assert_eq!("Stop", Status::Stop.to_string());
    }

    #[test]
    fn state_test() {
        // 使用 format! 宏来将枚举对象转换为 string
        assert_eq!("OK", format!("{:?}", State::OK));
        assert_eq!("ERR", State::ERR.to_string());
    }
}
