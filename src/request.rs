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
    organization: String,
    project: String,
    team: String,   // TODO: make into a list, need to then make multiple queries
    resource_path: String,
    // api version?
    optional_params: Vec<OptionalParam>,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(resource_path: &str) -> Request<T> {
        Request {
            resource_path: resource_path.to_owned(),
            organization: String::from(""),
            project: String::from(""),
            team: String::from(""),
            optional_params: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn set_organization(mut self, organization: &str) -> Request<T> {
        self.organization = organization.to_owned();
        self
    }

    pub fn set_project(mut self, project: &str) -> Request<T> {
        self.project = project.to_owned();
        self
    }

    pub fn set_team(mut self, team: &str) -> Request<T> {
        self.team = team.to_owned();
        self
    }

    pub async fn send(self, client: &AzureDevopsClient) -> Result<T, reqwest::Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        // This should now live in a builder and be tested
        let mut uri = PathBuf::new();
        uri.push(&self.organization);
        uri.push(&self.project);
        uri.push(&self.team);
        uri.push("_apis");
        uri.push(&self.resource_path);
        uri.push("?");

        let mut uri = uri.to_string_lossy().to_owned().to_string();
        for param in self.optional_params {
            uri.push_str(&format!("{}={}&", param.name, param.value));
        }
        uri.push_str("api-version=5.1");

        // This is all that belongs in send
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
