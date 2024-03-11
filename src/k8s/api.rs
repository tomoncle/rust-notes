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


use reqwest::{Certificate, Identity};

use crate::k8s::models::HttpKubeConfig;

pub struct HttpClient {
    server: String,
    client: reqwest::blocking::Client,
}

impl HttpClient {
    pub fn new(http_config: HttpKubeConfig) -> Self {
        let client_bundle_cert = format!("{}{}", http_config.client_certificate_data, http_config.client_key_data);
        let ca_certificate = Certificate::from_pem(http_config.certificate_authority_data.as_bytes()).unwrap();
        let identity = Identity::from_pem(&client_bundle_cert.as_bytes()).unwrap();
        HttpClient {
            server: http_config.server,
            client: reqwest::blocking::Client::builder()
                .use_rustls_tls()// 启用tls配置
                .identity(identity)// 加载客户端证书和私钥
                .add_root_certificate(ca_certificate) // 加载CA证书
                .build()
                .unwrap(),
        }
    }

    pub fn healthy(&self) -> bool {
        self.client
            .get(&self.server)
            .send()
            .expect("健康检查失败!").status().is_success()
    }

    pub fn api_list(&self) -> String {
        let response = self.client
            .get(&self.server)
            .send()
            .expect("查询失败!");
        let json: serde_json::Value = response.json().unwrap();
        json.to_string()
    }
}


pub fn join_path(parts: &[&str]) -> String {
    let mut result = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index == 0 {
            result.push_str(part.trim_end_matches("/"));
        } else {
            result.push('/');
            result.push_str(part.trim_start_matches("/").trim_end_matches("/"));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const KUBE_CONFIG: &str = r###"apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSURCVENDQWUyZ0F3SUJBZ0lJWHBjREwrVTlJMEl3RFFZSktvWklodmNOQVFFTEJRQXdGVEVUTUJFR0ExVUUKQXhNS2EzVmlaWEp1WlhSbGN6QWVGdzB5TXpFd01UVXhOVE0xTWpCYUZ3MHpNekV3TVRJeE5UUXdNakJhTUJVeApFekFSQmdOVkJBTVRDbXQxWW1WeWJtVjBaWE13Z2dFaU1BMEdDU3FHU0liM0RRRUJBUVVBQTRJQkR3QXdnZ0VLCkFvSUJBUUREM0xzWkJFZ29kK29kMEl6c0l0VVdEN1hkbzZUY3FKa2dQMFZhbnVMMmpCZi9QUmVTM2kyNWxvMnMKRFcyM3VjM0FtOW1yb2xxeG8xZXhqSnltYlhTZzZwbVhCcEZVR2hVaW9DVHhPSk90TFdwR2FBVCtKT1dNQk8wcwoxRUordFZ3NmNkSk42eTE0QmNTWXNTcXUxL3phY0Z3NE10ckVMZUwzQUNrR0tlRDRuako3UXMySVFFUkVWeXNSCkhQNnl6ekx6cmNndzYwTnZPcm5ZNUIxd2V1dEZkUlJDYVdUV3FuS0lycXVkTy81K1hrMXJodTQvdGFDaGFIVmUKcFZFNTlITnF4emU5VkFrL0hzNWw5NGcwRFgyYnZGeTV0U0ZuT0ZEUHZuWjdlRU1rWmZNd2cxNjNPbUVMUzlQQwpFbHV1eU8xdjZDRTgzMjA2OEFWYWxFY1R5TkN4QWdNQkFBR2pXVEJYTUE0R0ExVWREd0VCL3dRRUF3SUNwREFQCkJnTlZIUk1CQWY4RUJUQURBUUgvTUIwR0ExVWREZ1FXQkJRMDJJQXpWQ25wMExNQXJTcml4dkFTT01HRURUQVYKQmdOVkhSRUVEakFNZ2dwcmRXSmxjbTVsZEdWek1BMEdDU3FHU0liM0RRRUJDd1VBQTRJQkFRQjZiODlRWmRGdgpna2llV0lvb2xFK2Jaenh6dENzRXg2UTVxNEM4aVJ2cW5UVkVsUzl6T3gwRnU2dEhxSk9YWVZSR3dHU0t1b2NFCmx4SUhBKzgzMnhRLzVEbUs5YkN1dFE2RkFRMkVFYkIyUnZwSDhxS0RHa3V5b3MwYU03QWE0clN3RDNYR2pmR2oKeHl1NVBxMFBxdlZGNFl1Y3RGaGlzVCtUOE1HcGo1N1QxWlpyMitidE0yWnJFY0dHanNaaDNYM1V2WENTUVBmeQpTOHB4d0c4YXh6Z1ZqcUk1WjdUYjkwWmJCVk1kNVN4YmZhcDJiZnEyQTlvc2hIWG1zMjBmTDg0YktoNTgzcFJVCjMwamc3eVpHZU5HbVRSSk9IUENkTVBjQnVCT3VNVjg0TnE0RzFmV1NtR0dic1pNRVgzU2hrbTdEejFkTGxkNlkKRmExMVcyZ1RmbHh1Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0K
    server: https://api-server.k8s.local:9443
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
    client-certificate-data: LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSURJVENDQWdtZ0F3SUJBZ0lJYW05QitGdlZtK3N3RFFZSktvWklodmNOQVFFTEJRQXdGVEVUTUJFR0ExVUUKQXhNS2EzVmlaWEp1WlhSbGN6QWVGdzB5TXpFd01UVXhOVE0xTWpCYUZ3MHlOREV3TVRReE5UUXdNalJhTURReApGekFWQmdOVkJBb1REbk41YzNSbGJUcHRZWE4wWlhKek1Sa3dGd1lEVlFRREV4QnJkV0psY201bGRHVnpMV0ZrCmJXbHVNSUlCSWpBTkJna3Foa2lHOXcwQkFRRUZBQU9DQVE4QU1JSUJDZ0tDQVFFQXpidzFNTk9mcUFvS3Z2VGYKejFIMjZzWFE3bVdQZ1d2UXpjQVBzR2YzZWwzU2dpWjVIdHdKZDZPZ3ltd3J3QUxsTjlnNDg2VlA5aktlK2kvUQpCY3hWVWlkQUF1UW9BR1hmSk95N21jOFhaRWxlNlNRWFVqLzgveW10RDZxN2t2amdVdTFjQkQ5ZGQ0aXJmTkZkClBsYVZCNTloSmIwbnV5ZUt6c0c1UWFtVXBFd0p5RGhMYzFtUkM1Yk9Ra3QxZ2Jpa2VNZGFaWkFOSVpucGVWQXkKb2w4c3VqR3ZYbVoxNllZUnNMTzFqcW1ZUFdPbnJhY2JiRFBJelFtRlQ5VTVyNHU5RTlTSTV3RWlFMW5nR3p3TQpQL1lCQ3hYQTVicStIZ244ckRycm1QZEZXZFF3Nmt0bTVrSWpjSXFOSUNIZzVIcVdNT05iQ1dsbFJxS3BxV0JqCmdSZE1nd0lEQVFBQm8xWXdWREFPQmdOVkhROEJBZjhFQkFNQ0JhQXdFd1lEVlIwbEJBd3dDZ1lJS3dZQkJRVUgKQXdJd0RBWURWUjBUQVFIL0JBSXdBREFmQmdOVkhTTUVHREFXZ0JRMDJJQXpWQ25wMExNQXJTcml4dkFTT01HRQpEVEFOQmdrcWhraUc5dzBCQVFzRkFBT0NBUUVBdm9UcmhEWmdXcmgrcHJ6emY3SzdGV25FVDU4WUJlRWZSK2F5ClZDcXJueFBNeE5ROG1kRlFpZWliQ3Q0aXgyM1E0bFlKZk83ejMwMEJFSUllYXNDdGNTMGVOVk5YZDYwQXRaaUMKVVdTWUFlL1c3aHQ0cFUvUWZ0K1F4SlR4anplamlpRHRXdDFzZTBNb2g5V0w4YkcrdmRqa1lycDVsdjNZMm0vRgoyU0hCRmRKSktiVFh3L1lGSGxQL2FEcm1BM3ZlbkZKMCtseHBxL0tvaVFDWnZzWjQ4QjEwZjFYYSt3andURDRvCjJzSXovZVNuWXczZmJyRU1wcXA3OUxPTUVTYVV2c1lxaVp6bTZMN2VUNG5OeVNNY1BxR3MyVkJJZHpHUi9hSDAKbmR5RnpZTzUvai9GTTZZS2xjdGNqWU1GbDZTWlZBKzhoTW9Fa1JsVVM5NVZRTmFKT0E9PQotLS0tLUVORCBDRVJUSUZJQ0FURS0tLS0tCg==
    client-key-data: LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlFb2dJQkFBS0NBUUVBemJ3MU1OT2ZxQW9LdnZUZnoxSDI2c1hRN21XUGdXdlF6Y0FQc0dmM2VsM1NnaVo1Ckh0d0pkNk9neW13cndBTGxOOWc0ODZWUDlqS2UraS9RQmN4VlVpZEFBdVFvQUdYZkpPeTdtYzhYWkVsZTZTUVgKVWovOC95bXRENnE3a3ZqZ1V1MWNCRDlkZDRpcmZORmRQbGFWQjU5aEpiMG51eWVLenNHNVFhbVVwRXdKeURoTApjMW1SQzViT1FrdDFnYmlrZU1kYVpaQU5JWm5wZVZBeW9sOHN1akd2WG1aMTZZWVJzTE8xanFtWVBXT25yYWNiCmJEUEl6UW1GVDlVNXI0dTlFOVNJNXdFaUUxbmdHendNUC9ZQkN4WEE1YnErSGduOHJEcnJtUGRGV2RRdzZrdG0KNWtJamNJcU5JQ0hnNUhxV01PTmJDV2xsUnFLcHFXQmpnUmRNZ3dJREFRQUJBb0lCQUQ1VVNjMGNJQitad3oydApGRzFNQ01HQjR0V0c0VjQxV3RJcTROWHgrWkVTQWFlNm5qRlNPbkFMbTdnWDQzMU5vMEl5K0xZZ0hFU2JlM3lTCnBiUkwvb0ZnTU0xdmNBc0cwZjd1MWJzNGVYN0JsOGFNVDZCMGtHS0JydE96UnhhMDNjbGg3ZUcvak1LZEhrODMKWXNkekZiMTBkTXNwTG5OTi9JaTlPaklCZlczMHE3RWIvUmE3RWl2TkZzK0lCRkVyUXY1NVRNNGk2dUw4b0xiagpKSjdWL1I1UzArbHM5ckNOazVIOHA2TWJISFB6QitnWnA2MXdZNGw1NU5OTC9iY202akdsa1kxUWliRjFhVzhiCnlDM3BMeWNkY0Y4OFBUdjNTTXJtTlMyRnkzNTl3UzFiaGU4M003NGV1VVM1Y2I0NEZBMnRNaHRCVjIyd3hxQzQKOWYwMDhoa0NnWUVBejJ2U0RVMktZQk1mcW53UzRtaXl5Wm0rbFF2V3RTaEdDQ3k0bFAzRVJSUjVSeEJHS3gxSQo5ZHJoTGNNemJEbWlsR1NEanFCVXhCUzJ5NTYrL0tiMFEwSU9zaVc4K3EzbmVXNlpDOWtsSjZLck1QMWRSWVU3CkNvazlMRmpSeHBURG1jNkExSFJjMU9rQWs0OE1EZnkrWDJNZVNleXhvZDlXdVAzYnA0WHlZZTBDZ1lFQS9ldE4KU3BTanZkVmRzV24vQlNOZWJsbXBDODdHU2RTK0FwaXRzaDQ3KzlKVkhKcXp2WkdpYVVRbUl2aUtXeDBmVU14OQppVk9xRjAwZkNCR1l6MzBkWlZ6MkFhQ1VnWVdRTEdvNE9ocVpDRllXUlcwZkV0QXFrZXhWODBySXNBN1A4bGRqCmlWaHhOdVE4cjdVRjZFTGJFSTFQNU1xZWlPc1VCNmNUVndLU1dpOENnWUJ5SmFIcEZEME1kam04OXA3bDNqeFYKMmJnYUNmUFVxaDh3TEh3LzhnOXEvTU5wS2ptVFRJQjBYRW9EaHlMcmNpUllpYUFrZzNOaW5RdWpydTduMTNFZworbDlnQm9palk4UzQvUFpZb04xam5JK2RYREZTS3VtOCtKMmpwQlY5dFN5WG5oOGp6QTJ5VmErdGhORmJlVFhECkJLY01nNmVWcjNUb1JUdjJTeTZDV1FLQmdBemY4SFJXb3hMT0Rja2dJY0VsOFBUUmNRUGJRU3lmTDk4NzFKclMKYzZJcUlReXBQM0hlUDRqczNxNHBRZ0txc0ZyTmM1UVIvTzdmaHpKR2lpUURLUWxwdEVPUzI5VFJhK1VFb2NNNApkQWY3TFhmcHQ0aWhxM2JvcjliOU5MM1FrblNzUDZ6SUlqMXhSeTUxbEU0WmpiZ0NJa25zaUN0VEdCbm5POURPCm5aTWJBb0dBV3BKUkI1bFBmSm1wanFuK25ENXYvL1d1TURxYjZGQW5iVFE0Z01IR0trOVV3S3JnLzc4QVRsaDUKN2hrWXNNYUV0b0d2ek5kNzdTeFZkZGhEZHNDZkpnZ3pDREtiWVZ1NXJMYThFTWNxQ1lZdWxLejBWZ2RSZmx6bwpLQXAzVC9xaDJhOGpxYWI3b3lOZ0psQmlpdHE1UTQzdytvcGx6VVQzVFJMd3BGZlBjamM9Ci0tLS0tRU5EIFJTQSBQUklWQVRFIEtFWS0tLS0tCg==
"###;

    #[test]
    fn join_path_test() {
        let url = join_path(&["https://localhost:6443", "v1", "pod/list"]);
        assert_eq!(url, "https://localhost:6443/v1/pod/list")
    }

    #[test]
    #[cfg(feature = "local_runtime")]
    fn healthy_test() {
        use kube::config::Kubeconfig;

        let kube_config = Kubeconfig::from_yaml(KUBE_CONFIG).unwrap();
        let config = HttpKubeConfig::from_kube_config(kube_config);
        let http_client = HttpClient::new(config.clone());
        assert_eq!(http_client.healthy(), true);
    }

    #[test]
    #[cfg(feature = "local_runtime")]
    fn version_test() {
        use kube::config::Kubeconfig;

        let kube_config = Kubeconfig::from_yaml(KUBE_CONFIG).unwrap();
        let config = HttpKubeConfig::from_kube_config(kube_config);
        let http_client = HttpClient::new(config.clone());
        let url = join_path(&[&http_client.server, "/version"]);
        println!("{}", http_client.client.get(url).send().unwrap().text().unwrap())
    }
}