use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::request::Method;
use crate::request::RequestBuilder;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItemClassificationNode {
    pub id: i64,
    pub identifier: String,
    pub name: String,
    pub structure_type: String,
    pub has_children: bool,
    pub children: Option<Vec<WorkItemClassificationNode>>,
    pub attributes: Option<Attributes>,
    pub path: String,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub start_date: String,
    pub finish_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub parent: Option<Parent>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub href: String,
}

// TODO: Path is an optional variable so shouldn't be required in the function call
// TODO: Could replace structure_group with an enum - look into strum crate for converting to string
pub fn create_or_update(organization: &str, project: &str, structure_group: &str, path: &str, body: &str) -> RequestBuilder<WorkItemClassificationNode> {
    let mut resource_path = PathBuf::new();
    resource_path.push("wit/classificationnodes");
    resource_path.push(structure_group);
    resource_path.push(path);

    RequestBuilder::<WorkItemClassificationNode>::new(Method::Post, resource_path.to_str().unwrap())
        .set_organization(organization)
        .set_project(project)
        .set_body(body)
}

pub fn get(organization: &str, project: &str, structure_group: &str, path: &str, body: &str) -> RequestBuilder<WorkItemClassificationNode> {
    let mut resource_path = PathBuf::new();
    resource_path.push("wit/classificationnodes");
    resource_path.push(structure_group);
    resource_path.push(path);

    RequestBuilder::<WorkItemClassificationNode>::new(Method::Get, resource_path.to_str().unwrap())
        .set_organization(organization)
        .set_project(project)
        .set_body(body)
}
