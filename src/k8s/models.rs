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

use std::path::Path;

use base64::Engine;
use kube::config::Kubeconfig;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

/// [`HttpKubeConfig`] represents information on how to connect to a remote Kubernetes cluster
///
/// Stored in `~/.kube/config` by default
///
/// From [`Kubeconfig`][kube::config::Kubeconfig]
/// and this will handle the difference between in-cluster deployment and local development.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct HttpKubeConfig {
    /// PEM-encoded certificate authority certificates. Overrides `certificate_authority`
    pub certificate_authority_data: String,

    /// The address of the kubernetes cluster (https://hostname:port).
    pub server: String,

    /// PEM-encoded data from a client cert file for TLS. Overrides `client_certificate`
    pub client_certificate_data: String,

    /// PEM-encoded data from a client key file for TLS. Overrides `client_key`
    pub client_key_data: String,
}

impl HttpKubeConfig {
    pub fn from_yaml(text: &str) -> Self {
        let kube_config = Kubeconfig::from_yaml(text).unwrap();
        HttpKubeConfig::from_kube_config(kube_config)
    }

    pub fn read_from<P: AsRef<Path>>(path: P) -> Self {
        let kube_config = Kubeconfig::read_from(path).unwrap();
        HttpKubeConfig::from_kube_config(kube_config)
    }

    fn from_kube_config(kube_config: Kubeconfig) -> Self {
        let mut http_kube_config = HttpKubeConfig {
            certificate_authority_data: String::new(),
            server: String::new(),
            client_certificate_data: String::new(),
            client_key_data: String::new(),
        };

        if let Some(cluster) = kube_config.clusters.first() {
            if let Some(ca) = &cluster
                .cluster
                .as_ref()
                .and_then(|c| c.certificate_authority_data.as_deref())
            {
                http_kube_config.certificate_authority_data =
                    HttpKubeConfig::decode(ca.to_string());
            }
            if let Some(server) = &cluster.cluster.as_ref().and_then(|c| c.server.as_ref()) {
                http_kube_config.server = server.to_string();
            }
        }

        if let Some(user) = kube_config.auth_infos.first() {
            if let Some(cert) = &user
                .auth_info
                .as_ref()
                .and_then(|u| u.client_certificate_data.as_deref())
            {
                http_kube_config.client_certificate_data = HttpKubeConfig::decode(cert.to_string());
            }
            if let Some(key) = &user.auth_info.as_ref().and_then(|u| {
                u.client_key_data
                    .as_ref()
                    .map(|secret| secret.expose_secret().as_str())
            }) {
                http_kube_config.client_key_data = HttpKubeConfig::decode(key.to_string());
            }
        }
        http_kube_config
    }

    fn decode(encode_data: String) -> String {
        String::from_utf8_lossy(
            &base64::engine::general_purpose::STANDARD
                .decode(encode_data.as_bytes())
                .unwrap(),
        )
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kube_config_init_test() {
        let config = r###"apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCi0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0=
    server: https://127.0.0.1:9443
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
    client-certificate-data: LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCi0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0=
    client-key-data: LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQotLS0tLUVORCBSU0EgUFJJVkFURSBLRVktLS0tLQ==
"###;
        let kube_config = Kubeconfig::from_yaml(config).unwrap();
        let config = HttpKubeConfig::from_kube_config(kube_config);
        assert_eq!(config.client_key_data.is_empty(), false);
        assert_eq!(config.certificate_authority_data.is_empty(), false);
        assert_eq!(config.client_certificate_data.is_empty(), false)
    }
}
