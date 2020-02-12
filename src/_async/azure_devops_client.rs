extern crate reqwest;
use std::path::PathBuf;
use url::Url;

pub struct AzureDevopsClient {
    pat: String,
    client: reqwest::Client,
}

impl AzureDevopsClient {
    pub fn new(pat: &str) -> AzureDevopsClient {
        AzureDevopsClient {
            pat: pat.to_owned(),
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    // pub async fn send(self, client: &AzureDevopsClient) -> Result<T, reqwest::Error>
    // where
    //     for<'de> T: Deserialize<'de>,
    // {
    //     // This should now live in a builder and be tested
    //     let mut uri = PathBuf::new();
    //     uri.push(&self.organization);
    //     uri.push(&self.project);
    //     uri.push(&self.team);
    //     uri.push("_apis");
    //     uri.push(&self.resource_path);
    //     uri.push("?");

    //     let mut uri = uri.to_string_lossy().to_owned().to_string();
    //     for query in self.queries {
    //         uri.push_str(&format!("{}={}&", query.name, query.value));
    //     }
    //     uri.push_str("api-version=5.1");

    //     // This is all that belongs in send
    //     let raw_response = client.get(uri).await?;
    //     let iterations_api_response: T = raw_response.json::<T>().await?;
    //     Ok(iterations_api_response)
    // }

    pub async fn get(&self, query: String) -> Result<reqwest::Response, reqwest::Error> {
        let mut uri = PathBuf::new();
        uri.push("https://dev.azure.com");
        uri.push(query);

        let uri = Url::parse(uri.as_path().to_str().unwrap()).unwrap();

        //println!("{}", uri.as_str());

        let pat = ":".to_owned() + &self.pat;
        let auth_header = "Basic ".to_owned() + &base64::encode(&pat);

        Ok(self
            .client
            .get(uri.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?)
    }

    // TODO: add support for POST
}
