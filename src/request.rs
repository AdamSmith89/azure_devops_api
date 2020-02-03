extern crate serde;
use serde::Deserialize;

use std::marker::PhantomData;
use std::path::PathBuf;

use crate::azure_devops_client::AzureDevopsClient;

struct OptionalParam {
    name: String,
    value: String,
}

pub struct Request<T> {
    base_query: String,
    optional_params: Vec<OptionalParam>,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(query: &str) -> Request<T> {
        Request {
            base_query: query.to_owned(),
            optional_params: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub async fn send(self, client: &AzureDevopsClient) -> Result<T, reqwest::Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut uri = PathBuf::new();
        uri.push(self.base_query);
        uri.push("?");
        let mut uri = uri.to_string_lossy().to_owned().to_string();
        for param in self.optional_params {
            uri.push_str(&format!("{}={}&", param.name, param.value));
        }
        uri.push_str("api-version=5.1");

        let raw_response = client.get(uri).await?;
        let iterations_api_response: T = raw_response.json::<T>().await?;
        Ok(iterations_api_response)
    }

    pub fn optional_param(mut self, name: &str, value: &str) -> Request<T> {
        self.optional_params.push(OptionalParam {
            name: name.to_owned(),
            value: value.to_owned(),
        });
        self
    }

    // TODO - set api version?
}
