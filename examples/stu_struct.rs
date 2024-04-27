
use std::borrow::Cow;
use chrono::{NaiveDateTime, Utc};

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
// 在你的例子中,使用 Cow<'a, str> 可以让 NewBlueprint 结构体更加灵活地接受字符串参数,
// 无需关心参数是引用还是拥有所有权。这有助于提高代码的可维护性和健壮性。
#[derive(Debug)]
pub struct NewBlueprint<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub user_id: Cow<'a, str>,
    pub config: Cow<'a, str>,
    create_time: Option<Cow<'a, NaiveDateTime>>,
    update_time: Option<Cow<'a, NaiveDateTime>>,
}

// 配置默认值
impl<'a> Default for NewBlueprint<'a> {
    fn default() -> Self {
        let now = Utc::now().naive_local();
        NewBlueprint {
            name: Cow::Owned("".to_string()),
            description: Cow::Owned("".to_string()),
            user_id: Cow::Owned("".to_string()),
            config: Cow::Owned("".to_string()),
            create_time: Some(Cow::Owned(now.clone())),
            update_time: Some(Cow::Owned(now)),
        }
    }
}

impl<'a> NewBlueprint<'a> {
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        description: impl Into<Cow<'a, str>>,
        user_id: impl Into<Cow<'a, str>>,
        config: impl Into<Cow<'a, str>>,
    ) -> Self {
        let now = NaiveDateTime::now();
        NewBlueprint {
            name: name.into(),
            description: description.into(),
            user_id: user_id.into(),
            config: config.into(),
            create_time: Some(Cow::Owned(now.clone())),
            update_time: Some(Cow::Owned(now)),
        }
    }

    pub fn with_create_time(mut self, create_time: Cow<'a, NaiveDateTime>) -> Self {
        self.create_time = Some(create_time);
        self
    }

    pub fn with_update_time(mut self, update_time: Cow<'a, NaiveDateTime>) -> Self {
        self.update_time = Some(update_time);
        self
    }
}
fn main() {
    // 使用Default的方式一
    let new_blueprint1 = NewBlueprint {
        name: Cow::Borrowed("My Blueprint"),
        description: Cow::Borrowed("This is a blueprint"),
        user_id: Cow::Borrowed("user123"),
        config: Cow::Borrowed(r#"{"key":"value"}"#),
        // // create_time and update_time will use the default values
        ..Default::default()
    };
    println!("{:?}",new_blueprint1);
    println!("{}", new_blueprint1.create_time.is_none());
    // 使用Default的方式二
    let mut new_blueprint2:NewBlueprint = Default::default();
    println!("{:?}",new_blueprint2);
    println!("{}", new_blueprint2.create_time.is_none());

    new_blueprint2.config = Cow::from(r#"{"key":"value"}"#);
    println!("{:?}",new_blueprint2);
}