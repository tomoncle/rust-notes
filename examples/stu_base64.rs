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

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

/// 定义了一个名为 CUSTOM_ENGINE 的常量，
/// 使用了自定义的 base64 编码引擎 GeneralPurpose，并指定了使用 URL_SAFE 字母表 并且 不进行填充。
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

///通用 base64 引擎。
///
/// - 它不使用矢量 CPU 指令，因此它可以在任何系统上运行。
/// - 它的速度相当快（~2-3GiB/s）。
/// - 但是，它不是恒定时间，因此容易受到定时侧信道攻击。对于加载加密密钥等，建议使用即将推出的常量时间实现。
/// - 相当于Linux: echo -n "hello world" | base64
fn general_purpose_base64_engine(binding: String) {
    let input = binding.as_bytes();
    let mut buf = String::new();

    general_purpose::STANDARD.encode_string(input, &mut buf);
    println!("***通用的 base64 引擎编码: [{}]", buf);

    let decoded = general_purpose::STANDARD.decode(buf.clone()).unwrap();
    println!(
        "***通用的 base64 引擎解码: [{}]",
        String::from_utf8_lossy(&decoded)
    );
}

/// 自定义配置通用 base64 引擎。
///
/// - 相当于Linux: echo "hello world" | base64
fn custom_base64_engine(binding: String) {
    let input = binding.as_bytes();
    let mut buf = String::new();

    CUSTOM_ENGINE.encode_string(input, &mut buf);
    println!("###自定义 base64 引擎编码: [{}]", buf);

    let decoded_custom = CUSTOM_ENGINE.decode(buf.clone()).unwrap();
    println!(
        "###自定义 base64 引擎解码: [{}]",
        String::from_utf8_lossy(&decoded_custom)
    );
}

fn fake_cert() -> String {
    let cert_data = r###"-----BEGIN CERTIFICATE-----
MIIDITCCAgmgAwIBAgIIam9B+FvVm+swDQYJKoZIhvcNAQELBQAwFTETMBEGA1UE
AxMKa3ViZXJuZXRlczAeFw0yMzEwMTUxNTM1MjBaFw0yNDEwMTQxNTQwMjRaMDQx
FzAVBgNVBAoTDnN5c3RlbTptYXN0ZXJzMRkwFwYDVQQDExBrdWJlcm5ldGVzLWFk
bWluMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzbw1MNOfqAoKvvTf
z1H26sXQ7mWPgWvQzcAPsGf3el3SgiZ5HtwJd6OgymwrwALlN9g486VP9jKe+i/Q
BcxVUidAAuQoAGXfJOy7mc8XZEle6SQXUj/8/ymtD6q7kvjgUu1cBD9dd4irfNFd
PlaVB59hJb0nuyeKzsG5QamUpEwJyDhLc1mRC5bOQkt1gbikeMdaZZANIZnpeVAy
ol8sujGvXmZ16YYRsLO1jqmYPWOnracbbDPIzQmFT9U5r4u9E9SI5wEiE1ngGzwM
P/YBCxXA5bq+Hgn8rDrrmPdFWdQw6ktm5kIjcIqNICHg5HqWMONbCWllRqKpqWBj
gRdMgwIDAQABo1YwVDAOBgNVHQ8BAf8EBAMCBaAwEwYDVR0lBAwwCgYIKwYBBQUH
AwIwDAYDVR0TAQH/BAIwADAfBgNVHSMEGDAWgBQ02IAzVCnp0LMArSrixvASOMGE
DTANBgkqhkiG9w0BAQsFAAOCAQEAvoTrhDZgWrh+przzf7K7FWnET58YBeEfR+ay
VCqrnxPMxNQ8mdFQieibCt4ix23Q4lYJfO7z300BEIIeasCtcS0eNVNXd60AtZiC
UWSYAe/W7ht4pU/Qft+QxJTxjzejiiDtWt1se0Moh9WL8bG+vdjkYrp5lv3Y2m/F
2SHBFdJJKbTXw/YFHlP/aDrmA3venFJ0+lxpq/KoiQCZvsZ48B10f1Xa+wjwTD4o
2sIz/eSnYw3fbrEMpqp79LOMESaUvsYqiZzm6L7eT4nNySMcPqGs2VBIdzGR/aH0
ndyFzYO5/j/FM6YKlctcjYMFl6SZVA+8hMoEkRlUS95VQNaJOA==
-----END CERTIFICATE-----
"###;
    cert_data.to_string()
}

fn fake_cert_data() -> String {
    "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSURCVENDQWUyZ0F3SUJBZ0lJWHBjREwrVTlJMEl3RFFZSktvWklodmNOQVFFTEJRQXdGVEVUTUJFR0ExVUUKQXhNS2EzVmlaWEp1WlhSbGN6QWVGdzB5TXpFd01UVXhOVE0xTWpCYUZ3MHpNekV3TVRJeE5UUXdNakJhTUJVeApFekFSQmdOVkJBTVRDbXQxWW1WeWJtVjBaWE13Z2dFaU1BMEdDU3FHU0liM0RRRUJBUVVBQTRJQkR3QXdnZ0VLCkFvSUJBUUREM0xzWkJFZ29kK29kMEl6c0l0VVdEN1hkbzZUY3FKa2dQMFZhbnVMMmpCZi9QUmVTM2kyNWxvMnMKRFcyM3VjM0FtOW1yb2xxeG8xZXhqSnltYlhTZzZwbVhCcEZVR2hVaW9DVHhPSk90TFdwR2FBVCtKT1dNQk8wcwoxRUordFZ3NmNkSk42eTE0QmNTWXNTcXUxL3phY0Z3NE10ckVMZUwzQUNrR0tlRDRuako3UXMySVFFUkVWeXNSCkhQNnl6ekx6cmNndzYwTnZPcm5ZNUIxd2V1dEZkUlJDYVdUV3FuS0lycXVkTy81K1hrMXJodTQvdGFDaGFIVmUKcFZFNTlITnF4emU5VkFrL0hzNWw5NGcwRFgyYnZGeTV0U0ZuT0ZEUHZuWjdlRU1rWmZNd2cxNjNPbUVMUzlQQwpFbHV1eU8xdjZDRTgzMjA2OEFWYWxFY1R5TkN4QWdNQkFBR2pXVEJYTUE0R0ExVWREd0VCL3dRRUF3SUNwREFQCkJnTlZIUk1CQWY4RUJUQURBUUgvTUIwR0ExVWREZ1FXQkJRMDJJQXpWQ25wMExNQXJTcml4dkFTT01HRURUQVYKQmdOVkhSRUVEakFNZ2dwcmRXSmxjbTVsZEdWek1BMEdDU3FHU0liM0RRRUJDd1VBQTRJQkFRQjZiODlRWmRGdgpna2llV0lvb2xFK2Jaenh6dENzRXg2UTVxNEM4aVJ2cW5UVkVsUzl6T3gwRnU2dEhxSk9YWVZSR3dHU0t1b2NFCmx4SUhBKzgzMnhRLzVEbUs5YkN1dFE2RkFRMkVFYkIyUnZwSDhxS0RHa3V5b3MwYU03QWE0clN3RDNYR2pmR2oKeHl1NVBxMFBxdlZGNFl1Y3RGaGlzVCtUOE1HcGo1N1QxWlpyMitidE0yWnJFY0dHanNaaDNYM1V2WENTUVBmeQpTOHB4d0c4YXh6Z1ZqcUk1WjdUYjkwWmJCVk1kNVN4YmZhcDJiZnEyQTlvc2hIWG1zMjBmTDg0YktoNTgzcFJVCjMwamc3eVpHZU5HbVRSSk9IUENkTVBjQnVCT3VNVjg0TnE0RzFmV1NtR0dic1pNRVgzU2hrbTdEejFkTGxkNlkKRmExMVcyZ1RmbHh1Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0K".to_string()
}

fn main() {
    let binding = fake_cert();
    general_purpose_base64_engine(binding.clone());
    custom_base64_engine(binding);

    let cert_data = fake_cert_data();
    let vec = general_purpose::STANDARD.decode(cert_data).unwrap();
    println!(
        ">>> 通用的 base64 引擎解码: [{}]",
        String::from_utf8_lossy(&vec)
    );
}
