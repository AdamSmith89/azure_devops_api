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
    organization: String,
    project: String,
    team: String,   // TODO: make into a list, need to then make multiple queries
    resource_path: String,
    // api version?
    queries: Vec<Query>,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(resource_path: &str) -> Request<T> {
        Request {
            resource_path: resource_path.to_owned(),
            organization: String::from(""),
            project: String::from(""),
            team: String::from(""),
            queries: Vec::new(),
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

    pub fn add_query(mut self, name: &str, value: &str) -> Request<T> {
        self.queries.push(Query {
            name: name.to_owned(),
            value: value.to_owned(),
        });
        self
    }

    // TODO - set api version?

    pub fn send(self, client: &AzureDevopsClient) -> Result<T, ApiError>
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

        let mut uri = uri.to_string_lossy().into_owned();
        for query in self.queries {
            uri.push_str(&format!("{}={}&", query.name, query.value));
        }
        uri.push_str("api-version=5.1");

        let response = client.get(uri)?;

        if let Err(err) = response.error_for_status_ref() {
            return Err(err)?;
        }

        Ok(response.json::<T>()?)
    }
}
