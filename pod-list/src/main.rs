use futures::{StreamExt, TryStreamExt};
use kube::ResourceExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{ListParams, WatchEvent},
    client::Client,
    Api, Error,
};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Hello, world!");

    // create a client using kube-config first, the env variables as a fallback
    let client = Client::try_default().await.expect("failed to create client");

    // namespaced request abstration (which takes in a client or context)
    let api: Api<Pod> = Api::namespaced(client, "kube-system");

    // use the api request to list pods in a given namespace
    api.list(&ListParams::default())
        .await
        .unwrap()
        .items
        .iter()
        .map(|pod| pod.name_any())
        .for_each(|name| info!("{}", name));

    let mut stream = api.watch(&ListParams::default(), "0").await?.boxed();

    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Added(pod) => info!("ADDED: {}", pod.name_any()),
            WatchEvent::Modified(pod) => info!("MODIFIED: {}", pod.name_any()),
            WatchEvent::Deleted(pod) => info!("DELETED: {}", pod.name_any()),
            WatchEvent::Error(e) => error!("ERROR: {} {} {}", e.code, e.message, e.status),
            _ => {}
        };
    }

    Ok(())
}
