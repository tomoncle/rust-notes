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

use std::time::SystemTime;

use chrono::{Duration, NaiveDateTime, Utc};

pub const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn local_system_time() -> SystemTime {
    SystemTime::now()
}

pub fn local_timestamp() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub fn local_timestamp_ms() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}

pub fn local_timestamp_ns() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos()
}

pub fn local_timestamp_us() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros()
}

pub fn local_timestamp_str() -> String {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
}

pub fn naive_date_time_from_str(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, TIME_FORMAT).ok()
}

pub fn naive_date_time_from_option(s: Option<String>) -> Option<NaiveDateTime> {
    s.as_ref().and_then(|s| naive_date_time_from_str(s))
}

pub fn naive_date_time_to_str(dt: &NaiveDateTime) -> String {
    dt.format(TIME_FORMAT).to_string()
}

pub fn get_cst_naive_date_time() -> NaiveDateTime {
    Utc::now().naive_local() + Duration::hours(8)
}