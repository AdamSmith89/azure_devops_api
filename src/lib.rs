pub mod _async;
pub mod blocking;
pub mod request;
pub mod work;
pub mod work_item_tracking;



fn foo() {
    let client = blocking::azure_devops_client::AzureDevopsClient::new("1234");

    let list_iterations = work::iterations::list("Avecto-VSTS", "PrivilegeGuard")
        .set_team("Client Team Axolotl")
        .send(&client).unwrap();
}