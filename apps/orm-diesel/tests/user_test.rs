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

use diesel::associations::HasTable;
use orm_diesel::model::user::{UpdateUser, UpdateUserBuilder, User};

#[test]
fn test_update_user_default() {
    // https://docs.rs/derive_builder/latest/derive_builder/

    // 编写测试用例
    let user: UpdateUser = Default::default();
    println!("user default: {:?}", user);
    assert_eq!(user.name, None);

    let build1: UpdateUser = UpdateUserBuilder::default()
        .build()
        .unwrap_or_default();
    println!("user builder default1 :{:?}", build1);
    assert_eq!(build1.name, None);

    let build2: UpdateUser = UpdateUserBuilder::default()
        .name(Some("test".to_string()))
        .build()
        .unwrap();
    ;
    println!("user builder default2 :{:?}", build2);
    assert_eq!(build2.name.unwrap_or_default(), "test");

}

#[test]
fn test_user_attr() {
    // 编写测试用例
    let table = User::table();
    println!("user: {:?}", table);
}