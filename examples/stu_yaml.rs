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


const KUBE_CONFIG: &str = r###"apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: LS0tLS1CRUdJTeG8xZXhqSnltYlhnZFLS0tLS0K
    server: https://172.16.10.220:6443
  name: kubernetes
contexts:
- context:
    cluster: kubernetes
    user: kubernetes-admin
  name: kubernetes-admin@kubernetes
current-context: kubernetes-admin@kubernetes
kind: Config
preferences: {}
users:
- name: kubernetes-admin
  user:
    client-certificate-data: LS0tLS1CRReE5URUc25FVDU4WUJlRWZSK2FSXovJZ0tCg==
    client-key-data: LS0tLSYWNiR0V0c0VjQxV3RJcTRIEtFWS0tLS0tCg==
"###;

fn main() {

    // 将YAML字符串解析为serde_yaml::Value
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(KUBE_CONFIG).unwrap();

    let mut ca = "";
    let mut cert = "";
    let mut cert_key = "";
    // 获取 certificate-authority-data、client-certificate-data 和 client-key-data 属性值
    if let Some(clusters) = yaml_value["clusters"].as_sequence() {
        for cluster in clusters {
            if let Some(cluster_map) = cluster.as_mapping() {
                if let Some(cluster_data) = cluster_map.get(&serde_yaml::Value::String("cluster".to_string())) {
                    if let Some(certificate_authority_data) = cluster_data["certificate-authority-data"].as_str() {
                        println!("certificate-authority-data: {}", !certificate_authority_data.is_empty());
                        ca = certificate_authority_data;
                    }
                }
            }
        }
    }

    if let Some(users) = yaml_value["users"].as_sequence() {
        for user in users {
            if let Some(user_map) = user.as_mapping() {
                if let Some(user_data) = user_map.get(&serde_yaml::Value::String("user".to_string())) {
                    if let Some(client_certificate_data) = user_data["client-certificate-data"].as_str() {
                        println!("client-certificate-data: {}", !client_certificate_data.is_empty());
                        cert = client_certificate_data;
                    }
                    if let Some(client_key_data) = user_data["client-key-data"].as_str() {
                        println!("client-key-data: {}", !client_key_data.is_empty());
                        cert_key = client_key_data;
                    }
                }
            }
        }
    }

    println!("\n\nCA: {}, \n\nCERT: {}, \n\nCERT-KEY: {}", ca, cert, cert_key)
}