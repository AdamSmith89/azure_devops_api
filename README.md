# azure_devops_api
Rust wrapper around the Azure DevOps Services REST API. Provides wrappers which are split to match the sections listed in the [API reference guide](https://docs.microsoft.com/en-us/rest/api/azure/devops/?view=azure-devops-rest-5.1).

# TODO
- Check examples are accurate and can be used as-is!
- Add more APIs
- [Add OAuth support](https://docs.microsoft.com/en-us/azure/devops/integrate/get-started/authentication/oauth?view=azure-devops)
  - `Bearer {access_token}` instead of `Basic {:PAT}`

# Usage
## Install

```rust
cargo install azure_devops_api
```

## Creating a Client
Currently only supports async communication, suggested approach is to use `tokio`.
```toml
[dependencies]
tokio = { version = "0.2.11", features = ["macros"] } 
```
Example of creating a client in an async `main`.
```rust
extern crate azure_devops_api;
use azure_devops_api::azure_devops_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = azure_devops_client::AzureDevopsClient::new("pat") // PAT specified here for now
        // An auth method is required - maybe this should be in the constructor then?
        .set_authorization(azure_devops_api::PAT, "pat") // NOT IMPLEMENTED YET
        // Optional - can be defaulted here for all queries
        // or overriden in each query
        .set_organization("organization")
        // Optional - can be defaulted here for all queries
        // or overriden in each query
        .set_project("project")
        // Optional - can be defaulted here for all queries
        // or overriden in each query
        .set_teams("team1") // OR .set_teams(["team1"]) // NOT IMPLEMENTED YET
        // Optional (will default to latest stable, 5.1) - maybe make this a per-query thing? Or override-able per query?
        .set_api_version(azure_devops_api::FIVE_ONE); // NOT IMPLEMENTED YET
}
```
### Authorization Methods
- Authorization via PATs is currently the only supported method.

## Queries
The suggested approach is to use the provided API wrappers. These simplify the queries and provide back deserialized JSON.
```rust
use azure_devops_api::work::iterations;

// setup client as above...

// Basic query example
let iterations: iterations::ListIterations = iterations::list().send(&client).await?;

// Example of using an optional parameter
let iterations: iterations::ListIterations = iterations::list()
    .optional_param("$timeframe", "current")
    .send(&client)
    .await?;

// Example of overriding config of AzureDevopsClient on a per-request basis : NOT IMPLEMENTED YET
let iterations: iterations::ListIterations = iterations::list()
    .set_organization("organization")
    .set_project("project")
    .set_teams(["team1"])
    .send(&client).await?;

// Output some interesting information about the iterations
for iteration in iterations.value {
    println!("{} : {} -> {}", iteration.name, iteration.attributes.start_date, iteration.attributes.finish_date);
}
```
<br>If required then a raw queries can be performed through the `AzureDevopsClient` itself.
```rust
let raw_json: String = client.get("api_query").await?;
// Example of overriding config of AzureDevopsClient - NOT IMPLEMENTED YET
let raw_json: String = client.get("organization", "project", ["team1"], "api_query").await?;
```
<br>The `api_query` should be everything after "_apis". For example, the URL for listing iterations is;
```
https://dev.azure.com/{organization}/{project}/{team}/_apis/work/teamsettings/iterations?api-version=5.1
```
Therefore the query string required for the `AzureDevopsClient` would be;
```
work/teamsettings/iterations?api-version=5.1
```

## Supported APIs
- Work
  - Iterations
    - [List](https://docs.microsoft.com/en-us/rest/api/azure/devops/work/iterations/list?view=azure-devops-rest-5.1)
- Work Item Tracking
  - Wiql
    - [Query By Id](https://docs.microsoft.com/en-us/rest/api/azure/devops/wit/wiql/query%20by%20id?view=azure-devops-rest-5.1)