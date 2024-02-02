// 导入被测试的代码
#[test]
fn test_some_function() {
    // 编写测试用例
    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
}