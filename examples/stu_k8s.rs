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
#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
async fn k8s_pods() -> Result<(), kube::Error> {
    use k8s_openapi::api::core::v1::Pod;
    use kube::api::ListParams;
    use kube::{Api, Client};

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::all(client);
    let list_params = ListParams::default();
    let list_pods = pods.list(&list_params).await?;
    for pod in list_pods.items {
        println!("\nPod 详情: \n{:?}", pod.spec.clone().unwrap().containers);
        println!(
            "\n测试Json: \n{}",
            serde_json::to_string_pretty(&pod).unwrap()
        );
        println!("\n测试yaml: \n{}", serde_yaml::to_string(&pod).unwrap());
    }
    Ok(())
}

#[allow(dead_code)] // 使用 #[allow(dead_code)] 属性来禁止编译器对未使用的代码发出警告
async fn k8s_info() -> Result<(), kube::Error> {
    use kube::Client;

    let client = Client::try_default().await?;
    println!(
        "\n***** default namespace *****: [{}]",
        client.default_namespace()
    );

    let k8s_version = client.apiserver_version().await?;
    println!(
        "\n******* K8S VERSION *********: \n{}",
        serde_yaml::to_string(&k8s_version).unwrap()
    );

    let api_groups = client.list_api_groups().await?;
    println!(
        "\n******* API  GROUPS *********: \n{}",
        serde_yaml::to_string(&api_groups).unwrap()
    );

    let api_versions = client.list_core_api_versions().await?;
    println!(
        "\n******* API VERSION *********: \n{}",
        serde_yaml::to_string(&api_versions).unwrap()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    k8s_info().await?;
    k8s_pods().await?;
    Ok(())
}
