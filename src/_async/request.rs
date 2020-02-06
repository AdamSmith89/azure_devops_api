extern crate serde;
use serde::Deserialize;

use std::marker::PhantomData;
use std::path::PathBuf;

//use crate::azure_devops_client::AzureDevopsClient;
use crate::request_trait::RequestSuper;

pub struct Request<T> {
    inner: RequestSuper<T>,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(resource_path: &str) -> Request<T> {
        Request {
            inner: RequestSuper::<T>::new(resource_path),
            phantom: PhantomData,
        }
    }

    // TODO - set api version?

    pub fn send() {
        println!("async send");
    }
    // pub async fn send(self, client: &AzureDevopsClient) -> Result<T, reqwest::Error>
    // where
    //     for<'de> T: Deserialize<'de>,
    // {
    //     // This should now live in a builder and be tested
    //     let mut uri = PathBuf::new();
    //     uri.push(&self.organization);
    //     uri.push(&self.project);
    //     uri.push(&self.team);
    //     uri.push("_apis");
    //     uri.push(&self.resource_path);
    //     uri.push("?");

    //     let mut uri = uri.to_string_lossy().to_owned().to_string();
    //     for query in self.queries {
    //         uri.push_str(&format!("{}={}&", query.name, query.value));
    //     }
    //     uri.push_str("api-version=5.1");

    //     // This is all that belongs in send
    //     let raw_response = client.get(uri).await?;
    //     let iterations_api_response: T = raw_response.json::<T>().await?;
    //     Ok(iterations_api_response)
    // }
}
