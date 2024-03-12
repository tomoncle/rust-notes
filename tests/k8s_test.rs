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

use rust_notes::k8s::api;

#[test]
fn join_path_test() {
    assert_eq!(
        api::join_path(&["https://127.0.0.1:6443", "api"]),
        "https://127.0.0.1:6443/api"
    )
}

#[cfg(test)]
mod k8s_api_test {
    use rust_notes::k8s::api::HttpClient;
    use rust_notes::k8s::models::HttpKubeConfig;

    const KUBE_CONFIG_FILE: &str = "E:\\etc\\k8s\\kubeconfig";

    fn check() -> bool {
        if let Ok(metadata) = std::fs::metadata(KUBE_CONFIG_FILE) {
            return metadata.is_file();
        }
        false
    }

    #[test]
    fn check_url() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        assert_eq!(http_client.url(&["api", "v1", "pods"]).ends_with("/api/v1/pods"), true);
    }

    #[test]
    fn apis_list() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        println!("{}", http_client.apis())
    }

    #[test]
    fn api_groups() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        let url = http_client.url(&["/api/v1"]);
        let response = http_client.client.get(url).send().unwrap();
        println!("{}", response.text().unwrap())
    }

    #[test]
    fn pods() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        // kubectl get po -n kube-system --field-selector=status.phase=Running
        // /api/v1/namespaces/kube-system/pods?fieldSelector=status.phase%3DRunning&limit=500
        //
        // kubectl get po -n kube-system -l tier=control-plane
        let url = http_client.url(&[
            "/api/v1/namespaces",
            "kube-system",
            "pods",
            "?",
            "labelSelector=tier%3Dcontrol-plane",
            "&",
            "limit=500",
        ]);
        let response = http_client.client.get(url).send().unwrap();
        println!("{}", response.text().unwrap())
    }

    #[test]
    fn namespaces() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        let url = http_client.url(&["/api/v1/namespaces?limit=500"]);
        let response = http_client.client.get(url).send().unwrap();
        println!("{}", response.text().unwrap());
    }

    #[test]
    fn nodes() {
        assert_eq!(check(), true);
        let config = HttpKubeConfig::read_from(KUBE_CONFIG_FILE);
        let http_client = HttpClient::new(config);
        let url = http_client.url(&["/api/v1/nodes?limit=500"]);
        let response = http_client.client.get(url).send().unwrap();
        println!("{}", response.text().unwrap());
    }
}