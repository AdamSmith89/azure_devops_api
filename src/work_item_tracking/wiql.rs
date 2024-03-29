use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::request::RequestBuilder;
use crate::request::Method;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub query_type: String,
    pub query_result_type: String,
    pub as_of: String,
    pub columns: Vec<Column>,
    pub work_items: Vec<WorkItemReference>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub reference_name: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItemReference {
    pub id: i64,
    pub url: String,
}

pub fn query_by_id(organization: &str, id: &str) -> RequestBuilder<QueryResult> {
    let mut resource_path = PathBuf::new();
    resource_path.push("wit/wiql");
    resource_path.push(id);
    
    RequestBuilder::<QueryResult>::new(Method::Get, resource_path.to_str().unwrap())
        .set_organization(organization)
}