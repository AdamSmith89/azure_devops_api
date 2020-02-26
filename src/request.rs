use serde::Deserialize;
use std::marker::PhantomData;
use std::ops::Add;
use url::Url;

use crate::azure_devops_client::AzureDevopsClient;
use crate::errors::ApiError;

#[derive(PartialEq, Debug)]
pub enum Method {
    Delete,
    Get,
    Post,
}

#[derive(Debug)]
pub struct Request<T> {
    method: Method,
    url: Url,
    body: String,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(method: Method, url: Url, body: String) -> Request<T> {
        Request {
            method: method,
            url: url,
            body: body,
            phantom: PhantomData,
        }
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }

    pub fn send(self, client: &AzureDevopsClient) -> Result<T, ApiError>
    where
        for<'de> T: Deserialize<'de>,
    {
        let response = 
            match self.method {
                Method::Delete => client.delete(self.url)?,
                Method::Get => client.get(self.url)?,
                Method::Post => client.post(self.url, self.body)?,
            };

        if let Err(err) = response.error_for_status_ref() {
            return Err(err)?;
        }

        Ok(response.json::<T>()?)
    }
}

pub struct RequestBuilder<T> {
    method: Method,
    organization: String,
    project: String,
    team: String,
    resource_path: String,
    paths: Vec<String>,
    queries: Vec<(String, String)>,
    // api version?
    body: String,
    phantom: PhantomData<T>,
}

impl<T> RequestBuilder<T> {
    pub fn new(method: Method, resource_path: &str) -> RequestBuilder<T> {
        RequestBuilder {
            method: method,
            resource_path: resource_path.to_owned(),
            organization: String::new(),
            project: String::new(),
            team: String::new(),
            paths: Vec::new(),
            queries: Vec::new(),
            body: String::new(),
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

    pub fn add_path(mut self, path: &str) -> RequestBuilder<T> {
        self.paths.push(path.to_owned());
        self
    }
    
    pub fn add_query(mut self, name: &str, value: &str) -> RequestBuilder<T> {
        self.queries.push((name.to_owned(), value.to_owned()));
        self
    }

    pub fn set_body(mut self, body: &str) -> RequestBuilder<T> {
        self.body = body.to_owned();
        self
    }

    // TODO - set api version?

    pub fn build(self) -> Result<Request<T>, ApiError> {
        let mut url = Url::parse("https://dev.azure.com")?
            .join(&self.organization)?
            .join(&self.project)?
            .join(&self.team)?
            .join("_apis/")?
            .join(&self.resource_path)?;

        url.path_segments_mut().unwrap().extend(self.paths.into_iter());
        url.query_pairs_mut()
            .extend_pairs(self.queries.into_iter())
            .append_pair("api-version", "5.1");

        Ok(Request::new(self.method, url, self.body))
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
    use super::Method;
    use super::RequestBuilder;
    
    // TODO: Some tests for fail cases would be good

    #[test]
    fn requestbuilder_build_basic() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_resource?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_organization_only() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource").set_organization("fake_org");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/_apis/fake_resource?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_team_only() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource").set_team("fake_team");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_team/_apis/fake_resource?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_organization_and_team() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource")
            .set_organization("fake_org")
            .set_team("fake_team");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/fake_team/_apis/fake_resource?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_path() {
        let request_builder =
            RequestBuilder::<i32>::new(Method::Get, "fake_resource").add_path("fake_path");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_resource/fake_path?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_multiple_paths() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource")
            .add_path("fake_path1")
            .add_path("fake_path2")
            .add_path("fake_path3");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_resource/fake_path1/fake_path2/fake_path3?api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_query() {
        let request_builder =
            RequestBuilder::<i32>::new(Method::Get, "fake_resource").add_query("fake_query", "fake_value");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_resource?fake_query=fake_value&api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_with_multiple_queries() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource")
            .add_query("fake_query1", "fake_value1")
            .add_query("fake_query2", "fake_value2")
            .add_query("fake_query3", "fake_value3");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/_apis/fake_resource?fake_query1=fake_value1&fake_query2=fake_value2&fake_query3=fake_value3&api-version=5.1"
        );
    }

    #[test]
    fn requestbuilder_build_all_segments() {
        let request_builder = RequestBuilder::<i32>::new(Method::Get, "fake_resource")
            .set_organization("fake_org")
            .set_team("fake_team")
            .add_query("fake_query1", "fake_value1")
            .add_query("fake_query2", "fake_value2")
            .add_query("fake_query3", "fake_value3");
        let actual_request = request_builder.build();

        assert_eq!(
            actual_request.unwrap().url.as_str(),
            "https://dev.azure.com/fake_org/fake_team/_apis/fake_resource?fake_query1=fake_value1&fake_query2=fake_value2&fake_query3=fake_value3&api-version=5.1"
        );
    }
}
