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
}
