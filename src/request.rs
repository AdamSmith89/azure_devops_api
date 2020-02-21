use serde::Deserialize;
use std::marker::PhantomData;
use std::ops::Add;
use url::Url;

use crate::azure_devops_client::AzureDevopsClient;
use crate::errors::ApiError;

#[derive(Debug)]
pub struct Request<T> {
    url: Url,
    phantom: PhantomData<T>,
    // type: Get/Post?
}

impl<T> Request<T> {
    pub fn new(url: Url) -> Request<T> {
        Request {
            url: url,
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

struct Query {
    name: String,
    value: String,
}

pub struct RequestBuilder<T> {
    organization: String,
    project: String,
    team: String,
    resource_path: String,
    // type: Get/Post
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
        self.organization = organization.to_owned().add("/");
        self
    }

    pub fn set_project(mut self, project: &str) -> RequestBuilder<T> {
        self.project = project.to_owned().add("/");
        self
    }

    pub fn set_team(mut self, team: &str) -> RequestBuilder<T> {
        self.team = team.to_owned().add("/");
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

    pub fn build(mut self) -> Result<Request<T>, ApiError> {
        self.queries.push(Query {
            name: String::from("api-version"),
            value: String::from("5.1"),
        });

        let mut url = Url::parse("https://dev.azure.com")?
            .join(&self.organization)?
            .join(&self.project)?
            .join(&self.team)?
            .join("_apis/")?
            .join(&self.resource_path)?;

        for query in self.queries {
            url.query_pairs_mut().append_pair(&query.name, &query.value);
        }

        Ok(Request::new(url))
    }

    // Shortcut to call .build() then Request::Send()
    pub fn send(self, client: &AzureDevopsClient) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        self.build()?.send(client)
    }
}

#[cfg(test)]
mod tests {
    use super::RequestBuilder;
    
    // TODO: Some tests for fail cases would be good

    #[test]
    fn requestbuilder_build_basic() {
        let request_builder = RequestBuilder::<i32>::new("fake_path");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_path?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_organization_only() {
        let request_builder = RequestBuilder::<i32>::new("fake_path").set_organization("fake_org");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/_apis/fake_path?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_team_only() {
        let request_builder = RequestBuilder::<i32>::new("fake_path").set_team("fake_team");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_team/_apis/fake_path?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_organization_and_team() {
        let request_builder = RequestBuilder::<i32>::new("fake_path")
            .set_organization("fake_org")
            .set_team("fake_team");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/fake_team/_apis/fake_path?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_query() {
        let request_builder =
            RequestBuilder::<i32>::new("fake_path").add_query("fake_query", "fake_value");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_path?fake_query=fake_value&api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_multiple_queries() {
        let request_builder = RequestBuilder::<i32>::new("fake_path")
            .add_query("fake_query1", "fake_value1")
            .add_query("fake_query2", "fake_value2")
            .add_query("fake_query3", "fake_value3");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_path?fake_query1=fake_value1&fake_query2=fake_value2&fake_query3=fake_value3&api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_all_segments() {
        let request_builder = RequestBuilder::<i32>::new("fake_path")
            .set_organization("fake_org")
            .set_team("fake_team")
            .add_query("fake_query1", "fake_value1")
            .add_query("fake_query2", "fake_value2")
            .add_query("fake_query3", "fake_value3");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/fake_team/_apis/fake_path?fake_query1=fake_value1&fake_query2=fake_value2&fake_query3=fake_value3&api-version=5.1"
        );
    }
}
