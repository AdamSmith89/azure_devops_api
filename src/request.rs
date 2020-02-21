use serde::Deserialize;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::azure_devops_client::AzureDevopsClient;
use crate::errors::ApiError;

struct Query {
    name: String,
    value: String,
}

pub struct Request<T> {
    url: String,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(url: &str) -> Request<T> {
        Request {
            url: url.to_owned(),
            phantom: PhantomData,
        }
    }

    pub fn send(self, client: &AzureDevopsClient) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let response = client.get(self.url)?;

        if let Err(err) = response.error_for_status_ref() {
            return Err(err)?;
        }

        Ok(response.json::<T>()?)
    }
}

pub struct RequestBuilder<T> {
    organization: String,
    project: String,
    team: String,   // TODO: make into a list, need to then make multiple queries
    resource_path: String,
    // api version?
    queries: Vec<Query>,
    phantom: PhantomData<T>,
}

impl<T> RequestBuilder<T> {
    pub fn new(resource_path: &str) -> RequestBuilder<T> {
        RequestBuilder {
            resource_path: resource_path.to_owned(),
            organization: String::from(""),
            project: String::from(""),
            team: String::from(""),
            queries: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn set_organization(mut self, organization: &str) -> RequestBuilder<T> {
        self.organization = organization.to_owned();
        self
    }

    pub fn set_project(mut self, project: &str) -> RequestBuilder<T> {
        self.project = project.to_owned();
        self
    }

    pub fn set_team(mut self, team: &str) -> RequestBuilder<T> {
        self.team = team.to_owned();
        self
    }

    pub fn add_query(mut self, name: &str, value: &str) -> RequestBuilder<T> {
        self.queries.push(Query {
            name: name.to_owned(),
            value: value.to_owned(),
        });
        self
    }

    // TODO - set api version?

    pub fn build(self) -> Request<T> {
        let mut url = PathBuf::new();
        url.push(&self.organization);
        url.push(&self.project);
        url.push(&self.team);
        url.push("_apis");
        url.push(&self.resource_path);
        url.push("?");

        let mut url = url.to_string_lossy().into_owned();
        for query in self.queries {
            url.push_str(&format!("{}={}&", query.name, query.value));
        }
        url.push_str("api-version=5.1");

        Request::new(url.as_str())
    }

    // Shortcut to call .build() then Request::Send()
    pub fn send(self, client: &AzureDevopsClient) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        self.build().send(client)
    }
}