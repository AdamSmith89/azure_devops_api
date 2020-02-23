use serde::Deserialize;
use std::path::PathBuf;

use crate::request::Method;
use crate::request::RequestBuilder;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItemClassificationNode {
    pub id: i64,
    pub identifier: String,
    pub name: String,
    pub structure_type: String,
    pub has_children: bool,
    pub attributes: Option<Attributes>,
    pub path: String,
    #[serde(rename = "_links")]
    pub links: Links,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub start_date: String,
    pub finish_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Self_field,
    pub parent: Parent,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub href: String,
}

// Currently not supporting {path} - not sure what it's used for!
// TODO: Take request body parameters are required field. Need to work out how
// to get multiple optional body fields into this function...
// TODO: Could replace structure_group with an enum - look into strum crate for converting to string
pub fn create_or_update(organization: &str, project: &str, structure_group: &str, body: &str) -> RequestBuilder<WorkItemClassificationNode> {
    let mut resource_path = PathBuf::new();
    resource_path.push("wit/classificationnodes");
    resource_path.push(structure_group);

    RequestBuilder::<WorkItemClassificationNode>::new(Method::Post, resource_path.to_str().unwrap())
        .set_organization(organization)
        .set_project(project)
        .set_body(body)
}