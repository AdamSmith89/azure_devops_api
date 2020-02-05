extern crate reqwest;
extern crate serde;
use serde::Deserialize;

use std::path::PathBuf;

use crate::request::Request;

// TODO: How do we make the custom fields generic???

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkItems {
    pub count: i64,
    pub value: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub id: i64,
    pub rev: i64,
    pub fields: Fields,
    pub url: String,
    pub comment_version_ref: Option<CommentVersionRef>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    #[serde(rename = "System.AreaPath")]
    pub system_area_path: String,
    #[serde(rename = "System.TeamProject")]
    pub system_team_project: String,
    #[serde(rename = "System.IterationPath")]
    pub system_iteration_path: String,
    #[serde(rename = "System.WorkItemType")]
    pub system_work_item_type: String,
    #[serde(rename = "System.State")]
    pub system_state: String,
    #[serde(rename = "System.Reason")]
    pub system_reason: String,
    #[serde(rename = "System.CreatedDate")]
    pub system_created_date: String,
    #[serde(rename = "System.CreatedBy")]
    pub system_created_by: SystemCreatedBy,
    #[serde(rename = "System.ChangedDate")]
    pub system_changed_date: String,
    #[serde(rename = "System.ChangedBy")]
    pub system_changed_by: SystemChangedBy,
    #[serde(rename = "System.CommentCount")]
    pub system_comment_count: i64,
    #[serde(rename = "System.Title")]
    pub system_title: String,
    #[serde(rename = "System.BoardColumn")]
    pub system_board_column: Option<String>,
    #[serde(rename = "System.BoardColumnDone")]
    pub system_board_column_done: Option<bool>,
    #[serde(rename = "Microsoft.VSTS.Common.StateChangeDate")]
    pub microsoft_vsts_common_state_change_date: String,
    #[serde(rename = "Microsoft.VSTS.Common.Priority")]
    pub microsoft_vsts_common_priority: i64,
    #[serde(rename = "Microsoft.VSTS.Build.FoundIn")]
    pub microsoft_vsts_build_found_in: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Common.BacklogPriority")]
    pub microsoft_vsts_common_backlog_priority: Option<f64>,
    #[serde(rename = "Microsoft.VSTS.Common.Severity")]
    pub microsoft_vsts_common_severity: String,
    #[serde(rename = "Microsoft.VSTS.Common.ValueArea")]
    pub microsoft_vsts_common_value_area: String,
    #[serde(rename = "Microsoft.VSTS.TCM.SystemInfo")]
    pub microsoft_vsts_tcm_system_info: String,
    #[serde(rename = "Microsoft.VSTS.TCM.ReproSteps")]
    pub microsoft_vsts_tcm_repro_steps: String,
    #[serde(rename = "Microsoft.VSTS.Common.AcceptanceCriteria")]
    pub microsoft_vsts_common_acceptance_criteria: String,
    #[serde(rename = "System.History")]
    pub system_history: Option<String>,
    #[serde(rename = "System.Tags")]
    pub system_tags: Option<String>,
    #[serde(rename = "System.AssignedTo")]
    pub system_assigned_to: Option<SystemAssignedTo>,
    #[serde(rename = "Microsoft.VSTS.Common.ClosedDate")]
    pub microsoft_vsts_common_closed_date: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.Effort")]
    pub microsoft_vsts_scheduling_effort: Option<f64>,
    #[serde(rename = "System.Description")]
    pub system_description: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Build.IntegrationBuild")]
    pub microsoft_vsts_build_integration_build: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemCreatedBy {
    pub display_name: String,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: String,
    pub unique_name: String,
    pub image_url: String,
    pub inactive: Option<bool>,
    pub descriptor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub avatar: Avatar,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemChangedBy {
    pub display_name: String,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links2,
    pub id: String,
    pub unique_name: String,
    pub image_url: String,
    pub inactive: Option<bool>,
    pub descriptor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links2 {
    pub avatar: Avatar2,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemAssignedTo {
    pub display_name: String,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links3,
    pub id: String,
    pub unique_name: String,
    pub image_url: String,
    pub inactive: Option<bool>,
    pub descriptor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links3 {
    pub avatar: Avatar3,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar3 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentVersionRef {
    pub comment_id: i64,
    pub version: i64,
    pub url: String,
}


// Potential helper taking a vector of ints?
//pub fn list(organization: &str, ids: Vec<i32>) -> Request<ListWorkItems> {
    //.add_query("ids", ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))

// ids should be in CSV format
pub fn list(organization: &str, ids: String) -> Request<ListWorkItems> {
    let mut resource_path = PathBuf::new();
    resource_path.push("wit/workitems");
    
    Request::<ListWorkItems>::new(resource_path.to_str().unwrap())
        .set_organization(organization)
        .add_query("ids", ids.as_str())
}