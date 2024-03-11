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

// 参考文档：https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html
//
// 这里我们提供一个简单的参考，用来解释模块、路径、use关键词和pub关键词如何在编译器中工作，
// 以及大部分开发者如何组织他们的代码。我们将在本章节中举例说明每条规则，不过这是一个解释模块工作方式的良好参考。
//
// 从 crate 根节点开始: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是src/lib.rs，
// 对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。
//
// 声明模块:
//      在 crate 根文件中，你可以声明一个新模块；比如，你用 mod garden 声明了一个叫做 garden 的模块。
//      编译器会在下列路径中寻找模块代码：
//          1.内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
//          2.在文件 src/garden.rs
//          3.在文件 src/garden/mod.rs
//
// 声明子模块:
//      在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在src/garden.rs中定义了mod vegetables;。
//      编译器会在以父模块命名的目录中寻找子模块代码：
//          1.内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
//          2.在文件 src/garden/vegetables.rs
//          3.在文件 src/garden/vegetables/mod.rs
//
// 模块中的代码路径:
//      一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，
//      通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的 Asparagus 类型
//      可以在 crate::garden::vegetables::Asparagus 被找到。
//
// 私有 vs 公用:
//      一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。
//      为了使一个公用模块内部的成员公用，应当在声明前使用pub。
//
// use 关键字:
//      在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。
//      在任何可以引用 crate::garden::vegetables::Asparagus的作用域，
//      你可以通过 use crate::garden::vegetables::Asparagus; 创建一个快捷方式，
//      然后你就可以在作用域中只写 Asparagus 来使用该类型。
//
//
//
// 当正确配置了utils模块时(即存在 src/utils/mod.rs 或 src/utils.rs), 在 src/main.rs 中使用如下：
//
// ```
// // 引用utils的模块下的结构体和相关函数
// use utils::json::JsonConverter;
// use utils::fake_structs::Person;
//
// // 声明了一个叫做utils的模块
// mod utils;
// ```
//
// 假如没有 src/utils/mod.rs文件，也没有 src/utils.rs 文件， src/main.rs 是无法直接引用 utils 模块的，因为
//
// 编译器找不到名为 utils 的模块（即不存在 src/utils/mod.rs 或 src/utils.rs）,
// 那可使用下面方法引用 src/utils/json.rs 中的代码
//
// ```
// // 声明模块路径
// mod utils { pub mod json; pub mod fake_structs;}
//
// // 引用 JsonConverter 结构体和相关函数
// use utils::json::JsonConverter;
// use utils::fake_structs::Person;
//```


// 表示：当前 utils 模块包含了在 src/utils/json.rs 中的代码
pub mod json;
// 表示：当前 utils 模块包含了在 src/utils/fake_structs.rs 中的代码
pub mod fake_structs;


// 定义自己的 模块树
#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
pub mod country {
    pub mod china {
        pub fn zh() -> &'static str {
            "中国"
        }

        pub(crate) fn en() -> &'static str {
            "CHINA"
        }
    }

    pub mod usa {
        pub(crate) fn zh() -> &'static str {
            "美国"
        }

        pub fn en() -> &'static str {
            "USA"
        }
    }
}

#[cfg(test)]
mod tests {
    // 使用 use super::*; 将父模块中的所有内容引入到当前作用域中，这样就可以非常简单的实现对私有函数的测试。
    use super::*;

    #[test]
    fn country() {
        assert_eq!(country::usa::zh(), "美国");
        assert_eq!(country::china::en(), "CHINA");
    }
}