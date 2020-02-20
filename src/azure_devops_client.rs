use std::path::PathBuf;
use url::Url;

use crate::errors::ApiError;

pub struct AzureDevopsClient {
    pat: String,
    client: reqwest::blocking::Client,
}

impl AzureDevopsClient {
    pub fn new(pat: &str) -> AzureDevopsClient {
        AzureDevopsClient {
            pat: pat.to_owned(),
            client: reqwest::blocking::Client::builder()
                .redirect(reqwest::redirect::Policy::limited(1))
                .build()
                .unwrap(),
        }
    }

    pub fn get(&self, query: String) -> Result<reqwest::blocking::Response, ApiError> {
        let mut uri = PathBuf::new();
        uri.push("https://dev.azure.com");
        uri.push(query);

        let uri = Url::parse(uri.to_string_lossy().into_owned().as_str())?;

        let pat = ":".to_owned() + &self.pat;
        let auth_header = "Basic ".to_owned() + &base64::encode(&pat);

        Ok(self.client
            .get(uri.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()?)
    }

    // TODO: add support for POST
}
