use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::errors::ApiError;
use crate::request::Method;
use crate::request::RequestBuilder;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIterations {
    pub count: i64,
    #[serde(rename = "value")]
    pub iterations: Vec<Iteration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Iteration {
    pub id: String,
    pub name: String,
    pub path: String,
    pub attributes: Attributes,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
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

pub fn post_team_iteration(organization: &str, project: &str, id: &str) -> Result<RequestBuilder<Iteration>, ApiError> {
    Ok(
        RequestBuilder::<Iteration>::new(Method::Post, "work/teamsettings/iterations")
            .set_organization(organization)
            .set_project(project)
            .set_body(format!("{{\"id\":\"{}\"}}", id).as_str())
            //serde_json::to_string(&iteration)?.as_str()
    )
}
 