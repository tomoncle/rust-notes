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


use std::borrow::Cow;

use chrono::{NaiveDateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Cow<'a, str> 是 Rust 标准库中定义的一种类型,它代表了 "Borrowed or Owned" 字符串。
// 它可以存储一个实现了 ToOwned 和 Borrow trait 的值,并且在需要时自动进行所有权的转换
//
// 具体来说:
//
// Cow<'a, str> 可以存储一个 &'a str 类型的引用(借用)或者一个 String 类型的owned值。
//
// 当你需要一个 &str 时,如果内部存储的是 &'a str,它会直接返回该引用;
// 如果内部存储的是 String,它会自动借用该 String 的内容。
//
// 当你需要一个 String 时,如果内部存储的是 &'a str,它会自动克隆一个 String 返回;
// 如果内部存储的是 String,它会直接返回该 String。
//
// 使用 Cow<'a, str> 的好处是:
//
// 它可以灵活地处理字符串所有权,避免不必要的复制和分配。
// 它提供了统一的接口,使得代码更加通用和可复用。
// 在你的例子中,使用 Cow<'a, str> 可以让 UserInfo 结构体更加灵活地接受字符串参数,
// 无需关心参数是引用还是拥有所有权。这有助于提高代码的可维护性和健壮性。
#[derive(Debug)]
pub struct UserInfo<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub user_id: Cow<'a, str>,
    pub config: Cow<'a, str>,
    create_time: Option<Cow<'a, NaiveDateTime>>,
    update_time: Option<Cow<'a, NaiveDateTime>>,
}

// 自定义默认值
impl<'a> Default for UserInfo<'a> {
    fn default() -> Self {
        let now = Utc::now().naive_local();
        UserInfo {
            name: Cow::Owned("".to_string()),
            description: Cow::Owned("".to_string()),
            user_id: Cow::Owned("".to_string()),
            config: Cow::Owned("".to_string()),
            create_time: Some(Cow::Owned(now.clone())),
            update_time: Some(Cow::Owned(now)),
        }
    }
}

// 使用 UserInfo 初始化对象，只能传入pub类型的属性值，私有属性值，通过 with_xxx 函数配置
impl<'a> UserInfo<'a> {
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        description: impl Into<Cow<'a, str>>,
        user_id: impl Into<Cow<'a, str>>,
        config: impl Into<Cow<'a, str>>,
    ) -> Self {
        let now = Utc::now().naive_local();
        UserInfo {
            name: name.into(),
            description: description.into(),
            user_id: user_id.into(),
            config: config.into(),
            create_time: Some(Cow::Owned(now.clone())),
            update_time: Some(Cow::Owned(now)),
        }
    }

    // 私有属性值，通过 with_xxx 函数配置
    pub fn with_create_time(mut self, create_time: Cow<'a, NaiveDateTime>) -> Self {
        self.create_time = Some(create_time);
        self
    }

    // 私有属性值，通过 with_xxx 函数配置
    pub fn with_update_time(mut self, update_time: Cow<'a, NaiveDateTime>) -> Self {
        self.update_time = Some(update_time);
        self
    }
}

// Debug      : 允许使用 {:?} 格式打印结构体的内容。
// Default    : 允许使用 Default::default() 创建结构体的默认实例。
// Builder    : 允许使用 Builder 模式创建结构体实例。需要配置外部依赖 derive_builder 库
//
// #[builder(setter(into), default)] 是 derive_builder 库提供的一个属性,它可以应用在结构体的字段上,
// 用于控制 Builder 模式的行为。
//
// 这个属性有两个作用:
//
// 1. setter(into):
//
// 这个属性告诉 derive_builder 库,在使用 Builder 模式设置字段时,将传入的值自动转换为字段类型。
// 例如,如果字段类型是 Option<String>,那么在使用 Builder 设置时,
// 可以直接传入一个 String，它会被自动转换为 Some(String)。
//
// 2.default:
//
// 这个属性告诉 derive_builder 库,为这个字段生成一个默认值的 setter 方法。
// 当使用 Builder::default() 创建 Builder 实例时,这个字段会被设置为默认值。
#[derive(Debug, Default, Builder)]
#[builder(setter(into), default)]
struct Sample {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: Option<bool>,
}

// 测试 Default
#[derive(Debug, Default)]
pub struct ViewInfo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub config: Value,
    pub state: bool,
    pub create_time: Option<NaiveDateTime>,
}

fn main() {
    println!("使用单元测试测试结果")
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{Sample, SampleBuilder, UserInfo, ViewInfo};

    #[test]
    fn userinfo_default_test() {
        let user = UserInfo {
            name: Cow::Borrowed("tom"),
            description: Cow::Borrowed("developer"),
            user_id: Cow::Borrowed("123456"),
            config: Cow::Borrowed(r#"{"key":"value"}"#),
            // create_time and update_time will use the default values
            ..Default::default()
        };
        println!("{:?}", user);
        assert_eq!(user.create_time.is_some(), true)
    }

    #[test]
    fn userinfo_immutable_default_test() {
        let mut user: UserInfo = Default::default();
        println!("{:?}", user);
        assert_eq!(user.create_time.is_some(), true);

        user.config = Cow::from(r#"{"key":"value"}"#);
        println!("{:?}", user);
        assert_eq!(user.config.is_empty(), false);
    }

    #[test]
    fn sample_builder_default_test() {
        // https://docs.rs/derive_builder/latest/derive_builder/

        // 编写测试用例
        let sample: Sample = Default::default();
        println!("unBuilder default : {:?}", sample);
        assert_eq!(sample.name, None);

        let build1: Sample = SampleBuilder::default().build().unwrap_or_default();
        println!("isBuilder default1 :{:?}", build1);
        assert_eq!(build1.name, None);

        let build2: Sample = SampleBuilder::default()
            .name(Some("test".to_string()))
            .build()
            .unwrap();
        println!("isBuilder default2 :{:?}", build2);
        assert_eq!(build2.name.unwrap_or_default(), "test");
    }

    #[test]
    fn view_info_default_test() {
        // https://docs.rs/derive_builder/latest/derive_builder/

        // 编写测试用例
        let mut view_info: ViewInfo = Default::default();
        println!("default : {:?}", view_info);

        view_info.name = "hello".to_string();
        println!("default : {:?}", view_info);
    }
}