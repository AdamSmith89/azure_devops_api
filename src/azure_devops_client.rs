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

    // TODO: Can we still get the body of a response on a failed message?

    pub fn get(&self, url: Url) -> Result<reqwest::blocking::Response, ApiError> {
        let pat = ":".to_owned() + &self.pat;
        let auth_header = "Basic ".to_owned() + &base64::encode(&pat);

        Ok(self.client
            .get(url.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()?)
    }

    pub fn post(&self, url: Url, body: String) -> Result<reqwest::blocking::Response, ApiError> {
        let pat = ":".to_owned() + &self.pat;
        let auth_header = "Basic ".to_owned() + &base64::encode(&pat);

        Ok(self.client
            .post(url.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()?)
    }

    pub fn delete(&self, url: Url) -> Result<reqwest::blocking::Response, ApiError> {
        let pat = ":".to_owned() + &self.pat;
        let auth_header = "Basic ".to_owned() + &base64::encode(&pat);

        Ok(self.client
            .delete(url.as_str())
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()?)
    }
}
