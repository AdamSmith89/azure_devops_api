extern crate reqwest;
extern crate serde;
use serde::Deserialize;

use std::path::PathBuf;

use crate::request::Request;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub query_type: String,
    pub query_result_type: String,
    pub as_of: String,
    pub columns: Vec<Column>,
    pub work_items: Vec<WorkItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub reference_name: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkItem {
    pub id: i64,
    pub url: String,
}

pub fn query_by_id(organization: &str, id: &str) -> Request<QueryResult> {
    let mut query = PathBuf::new();
    query.push("wit/wiql");
    query.push(id);
    
    Request::<QueryResult>::new(query.to_str().unwrap())
        .set_organization(organization)
}