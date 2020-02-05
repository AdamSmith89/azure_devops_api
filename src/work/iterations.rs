extern crate reqwest;
extern crate serde;
use serde::Deserialize;

use std::path::PathBuf;

use crate::request::Request;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIterations {
    pub count: i64,
    pub value: Vec<Iteration>,
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
    pub start_date: String,
    pub finish_date: String,
    pub time_frame: String,
}

pub fn list(organization: &str, project: &str) -> Request<ListIterations> {
    Request::<ListIterations>::new("work/teamsettings/iterations")
        .set_organization(organization)
        .set_project(project)
}

pub fn get(organization: &str, project: &str, id: &str) -> Request<Iteration> {
    let mut query = PathBuf::new();
    query.push("work/teamsettings/iterations");
    query.push(id);
    
    Request::<Iteration>::new(query.to_str().unwrap())
        .set_organization(organization)
        .set_project(project)
}