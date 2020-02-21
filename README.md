# azure_devops_api
Rust wrapper around the Azure DevOps Services REST API. Provides wrappers which are split to match the services, categories and operations listed in the [API reference guide](https://docs.microsoft.com/en-us/rest/api/azure/devops/?view=azure-devops-rest-5.1).

# Usage
## Install

```toml
[dependencies]
azure_devops_api = "0.1.0"
```

## Creating a Client

```rust
extern crate azure_devops_api;
use azure_devops_api::azure_devops_client;

fn main() {
    let client = azure_devops_client::AzureDevopsClient::new("{pat}");
}
```
### Authorization
Authorization via PAT is currently the only supported method.

### Synchronicity
Currently only supports blocking requests.

## Queries
All wrapped APIs follow these rules;
- Modules map to the services and categories defined by the [Rest API docs](https://docs.microsoft.com/en-us/rest/api/azure/devops/?view=azure-devops-rest-5.1).
- Functions map to the operations defined by an individual service category.
- Required parameters for an operation are required in the corresponding function call.
- Optional parameters can be provided afterwards with the `Request::set_...` and `Request::add_query` functions.
  - Those which form part of the resource path should use `Request::set_...` functions.
  - Those which form part of the query should use the `Request::add_query` function.

To demonstrate these rules we can query the list of iterations belonging to a specific team. From the [Rest API docs for this operation](https://docs.microsoft.com/en-us/rest/api/azure/devops/work/iterations/list?view=azure-devops-rest-5.1) we can see the **List** operation lives in the **Iterations** category under the **Work** service, so the wrapper is;
```rust
azure_devops_api::work::iterations::list(...)
```
The URL to query for a specific teams iterations is;
```
https://dev.azure.com/{organization}/{project}/{team}/_apis/work/teamsettings/iterations?$timeframe={$timeframe}&api-version=5.1
```
In this instance the **team** and **$timeframe** parameters are listed as optional, so the wrapper call looks as follows;
```rust
// Required parameters provided in the function call
let list_iterations = azure_devops_api::work::iterations::list("{organization}", "{project}")
    // Optionally provide the team - forms part of the resource path so uses a "set_..." function
    .set_team("{team}")
    // Optionally provide the timeframe - is a query so uses the "add_query" function
    .add_query("$timeframe", "current")
    .send(&client)?;
```
The wrappers return deserialized JSON that maps the default fields returned by the API.
## Full Example
```rust
extern crate azure_devops_api;
use azure_devops_api::azure_devops_client::AzureDevopsClient;
use azure_devops_api::work::iterations;

fn main() {
    let client = AzureDevopsClient::new("{pat}");

    // Just using the required parameters, not providing the optional "team" and "timeframe" parameters
    let list_iterations = iterations::list("{organization}", "{project}").send(&client).unwrap();

    // Output some interesting information about the iterations
    for iteration in list_iterations.iterations {
        println!("{} : {} -> {}", iteration.name, iteration.attributes.start_date, iteration.attributes.finish_date);
    }
}
```
## Errors
??
## Supported APIs
- Work
  - Iterations
    - [List](https://docs.microsoft.com/en-us/rest/api/azure/devops/work/iterations/list?view=azure-devops-rest-5.1)
- Work Item Tracking
  - Wiql
    - [Query By Id](https://docs.microsoft.com/en-us/rest/api/azure/devops/wit/wiql/query%20by%20id?view=azure-devops-rest-5.1)
  - Work Items
    - [List](https://docs.microsoft.com/en-us/rest/api/azure/devops/wit/work%20items/list?view=azure-devops-rest-5.1)
## Unsupported APIs
APIs that have yet to be supported can be performed directly through the `AzureDevopsClient` itself. Deserialization will have to be performed manually.
```rust
let json: some_struct = client.get(Url::parse("https://dev.azure.com/...")?)?.json()?;
```
The `get` method takes a `url::Url` and returns a `Result<reqwest::blocking::response, ApiError>`. The [reqwest](https://crates.io/crates/reqwest) library supports deserialization through [serde_json](https://crates.io/crates/serde_json) using the `json` function.

# TODO
- Add support for post
- Support custom fields
- Add features for the different services/categories?
- Add more APIs
- [Add OAuth support](https://docs.microsoft.com/en-us/azure/devops/integrate/get-started/authentication/oauth?view=azure-devops)
  - `Bearer {access_token}` instead of `Basic {:PAT}`