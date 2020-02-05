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
    #[serde(rename = "Avecto.SalesForceId")]
    pub avecto_sales_force_id: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Common.BacklogPriority")]
    pub microsoft_vsts_common_backlog_priority: Option<f64>,
    #[serde(rename = "Microsoft.VSTS.Common.Severity")]
    pub microsoft_vsts_common_severity: String,
    #[serde(rename = "Avecto.Customers")]
    pub avecto_customers: Option<String>,
    #[serde(rename = "WEF_B4520A4C84F34E08977E19F7553378FA_Kanban.Column")]
    pub wef_b4520_a4_c84_f34_e08977_e19_f7553378_fa_kanban_column: Option<String>,
    #[serde(rename = "WEF_3985981C06D948A7A1187E78193571CD_Kanban.Column")]
    pub wef3985981_c06_d948_a7_a1187_e78193571_cd_kanban_column: Option<String>,
    #[serde(rename = "WEF_3985981C06D948A7A1187E78193571CD_Kanban.Column.Done")]
    pub wef3985981_c06_d948_a7_a1187_e78193571_cd_kanban_column_done: Option<bool>,
    #[serde(rename = "WEF_B4520A4C84F34E08977E19F7553378FA_Kanban.Column.Done")]
    pub wef_b4520_a4_c84_f34_e08977_e19_f7553378_fa_kanban_column_done: Option<bool>,
    #[serde(rename = "Microsoft.VSTS.Common.ValueArea")]
    pub microsoft_vsts_common_value_area: String,
    #[serde(rename = "WEF_82CAEDCD469444478729E65DAE484B36_Kanban.Column")]
    pub wef82_caedcd469444478729_e65_dae484_b36_kanban_column: Option<String>,
    #[serde(rename = "WEF_82CAEDCD469444478729E65DAE484B36_Kanban.Column.Done")]
    pub wef82_caedcd469444478729_e65_dae484_b36_kanban_column_done: Option<bool>,
    #[serde(rename = "Avecto.DefectCategorisation")]
    pub avecto_defect_categorisation: Option<String>,
    #[serde(rename = "Microsoft.VSTS.TCM.SystemInfo")]
    pub microsoft_vsts_tcm_system_info: String,
    #[serde(rename = "Microsoft.VSTS.TCM.ReproSteps")]
    pub microsoft_vsts_tcm_repro_steps: String,
    #[serde(rename = "Microsoft.VSTS.Common.AcceptanceCriteria")]
    pub microsoft_vsts_common_acceptance_criteria: String,
    #[serde(rename = "Avecto.Category")]
    pub avecto_category: Option<String>,
    #[serde(rename = "System.History")]
    pub system_history: Option<String>,
    #[serde(rename = "System.Tags")]
    pub system_tags: Option<String>,
    #[serde(rename = "Avecto.Documentation")]
    pub avecto_documentation: Option<String>,
    #[serde(rename = "System.AssignedTo")]
    pub system_assigned_to: Option<SystemAssignedTo>,
    #[serde(rename = "Microsoft.VSTS.Common.ClosedDate")]
    pub microsoft_vsts_common_closed_date: Option<String>,
    #[serde(rename = "Avecto.AssignedTester")]
    pub avecto_assigned_tester: Option<AvectoAssignedTester>,
    #[serde(rename = "Avecto.TestPlanRich")]
    pub avecto_test_plan_rich: Option<String>,
    #[serde(rename = "Avecto.VerifiedIn")]
    pub avecto_verified_in: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.Effort")]
    pub microsoft_vsts_scheduling_effort: Option<f64>,
    #[serde(rename = "WEF_3A9D73A5B2EA4C75BC858A930C26AED4_Kanban.Column")]
    pub wef3_a9_d73_a5_b2_ea4_c75_bc858_a930_c26_aed4_kanban_column: Option<String>,
    #[serde(rename = "WEF_3A9D73A5B2EA4C75BC858A930C26AED4_Kanban.Column.Done")]
    pub wef3_a9_d73_a5_b2_ea4_c75_bc858_a930_c26_aed4_kanban_column_done: Option<bool>,
    #[serde(rename = "Avecto.DevelopmentType")]
    pub avecto_development_type: Option<String>,
    #[serde(rename = "WEF_09F30A60F09B40389B573014BED5A3D2_Kanban.Column")]
    pub wef09_f30_a60_f09_b40389_b573014_bed5_a3_d2_kanban_column: Option<String>,
    #[serde(rename = "WEF_09F30A60F09B40389B573014BED5A3D2_Kanban.Column.Done")]
    pub wef09_f30_a60_f09_b40389_b573014_bed5_a3_d2_kanban_column_done: Option<bool>,
    #[serde(rename = "WEF_1EEDA8B39596405A9F3E980C4C6C8CB5_Kanban.Column")]
    pub wef1_eeda8_b39596405_a9_f3_e980_c4_c6_c8_cb5_kanban_column: Option<String>,
    #[serde(rename = "WEF_1EEDA8B39596405A9F3E980C4C6C8CB5_Kanban.Column.Done")]
    pub wef1_eeda8_b39596405_a9_f3_e980_c4_c6_c8_cb5_kanban_column_done: Option<bool>,
    #[serde(rename = "WEF_584C55DF027D4C82B131EB0C239556AD_Kanban.Column")]
    pub wef584_c55_df027_d4_c82_b131_eb0_c239556_ad_kanban_column: Option<String>,
    #[serde(rename = "WEF_584C55DF027D4C82B131EB0C239556AD_Kanban.Column.Done")]
    pub wef584_c55_df027_d4_c82_b131_eb0_c239556_ad_kanban_column_done: Option<bool>,
    #[serde(rename = "Avecto.TargetRelease")]
    pub avecto_target_release: Option<String>,
    #[serde(rename = "System.Description")]
    pub system_description: Option<String>,
    #[serde(rename = "WEF_217EFA852D77458EA84393573F477CBF_Kanban.Column")]
    pub wef217_efa852_d77458_ea84393573_f477_cbf_kanban_column: Option<String>,
    #[serde(rename = "WEF_217EFA852D77458EA84393573F477CBF_Kanban.Column.Done")]
    pub wef217_efa852_d77458_ea84393573_f477_cbf_kanban_column_done: Option<bool>,
    #[serde(rename = "WEF_95AA98A3EFA14A1CAED52F3740681F48_Kanban.Column")]
    pub wef95_aa98_a3_efa14_a1_caed52_f3740681_f48_kanban_column: Option<String>,
    #[serde(rename = "WEF_95AA98A3EFA14A1CAED52F3740681F48_Kanban.Column.Done")]
    pub wef95_aa98_a3_efa14_a1_caed52_f3740681_f48_kanban_column_done: Option<bool>,
    #[serde(rename = "Microsoft.VSTS.Build.IntegrationBuild")]
    pub microsoft_vsts_build_integration_build: Option<String>,
    #[serde(rename = "WEF_BC95EE99781C4AE5A37BB705E9649130_Kanban.Column")]
    pub wef_bc95_ee99781_c4_ae5_a37_bb705_e9649130_kanban_column: Option<String>,
    #[serde(rename = "WEF_BC95EE99781C4AE5A37BB705E9649130_Kanban.Column.Done")]
    pub wef_bc95_ee99781_c4_ae5_a37_bb705_e9649130_kanban_column_done: Option<bool>,
    #[serde(rename = "WEF_3BB3263D6BAB4FB5A85C81FF614EC356_Kanban.Column")]
    pub wef3_bb3263_d6_bab4_fb5_a85_c81_ff614_ec356_kanban_column: Option<String>,
    #[serde(rename = "WEF_3BB3263D6BAB4FB5A85C81FF614EC356_Kanban.Column.Done")]
    pub wef3_bb3263_d6_bab4_fb5_a85_c81_ff614_ec356_kanban_column_done: Option<bool>,
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
pub struct AvectoAssignedTester {
    pub display_name: String,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links4,
    pub id: String,
    pub unique_name: String,
    pub image_url: String,
    pub inactive: bool,
    pub descriptor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links4 {
    pub avatar: Avatar4,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar4 {
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