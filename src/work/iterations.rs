use serde::Deserialize;
use std::path::PathBuf;

use crate::request::RequestBuilder;
use crate::request::Method;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIterations {
    pub count: i64,
    #[serde(rename = "value")]
    pub iterations: Vec<Iteration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iteration {
    pub id: String,
    pub name: String,
    pub path: String,
    pub attributes: Attributes,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub time_frame: String,
}

pub fn list(organization: &str, project: &str) -> RequestBuilder<ListIterations> {
    RequestBuilder::<ListIterations>::new(Method::Get, "work/teamsettings/iterations")
        .set_organization(organization)
        .set_project(project)
}

pub fn get(organization: &str, project: &str, id: &str) -> RequestBuilder<Iteration> {
    let mut resource_path = PathBuf::new();
    resource_path.push("work/teamsettings/iterations");
    resource_path.push(id);
    RequestBuilder::<Iteration>::new(Method::Get, resource_path.to_str().unwrap())
        .set_organization(organization)
        .set_project(project)
}
