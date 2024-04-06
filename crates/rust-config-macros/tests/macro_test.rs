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
#[cfg(test)]
mod macro_lib_test {
    use rust_config_macros::sample_attribute;

    #[sample_attribute()]
    fn test(a: &str, b: i32) {
        println!("test: {} , {}", a, b);
    }

    #[sample_attribute(foo, bar)]
    fn test_foo(a: &str, b: i32) {
        println!("test: {} , {}", a, b);
    }

    #[sample_attribute(bar)]
    fn test_bar(a: &str, b: i32) {
        println!("test: {} , {}", a, b);
    }

    #[test]
    fn test_sample_attribute() {
        test("hello", 10);
        test_foo("hello", 20);
        test_bar("hello", 30);
    }
}
