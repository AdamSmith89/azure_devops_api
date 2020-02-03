use std::path::PathBuf;
use url::Url;

pub struct AzureDevopsClient {
    pat: String,
    organization: String, // TODO - make these override-able
    project: String,
    team: String,   // TODO: make into a list, need to then make multiple queries
    // api version?
    client: reqwest::Client,
}

impl AzureDevopsClient {
    pub fn new(pat: &str) -> AzureDevopsClient {
        AzureDevopsClient {
            pat: pat.to_owned(),
            organization: String::from(""),
            project: String::from(""),
            team: String::from(""),
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    pub fn set_organization(mut self, organization: &str) -> AzureDevopsClient {
        self.organization = organization.to_owned();
        self
    }

    pub fn set_project(mut self, project: &str) -> AzureDevopsClient {
        self.project = project.to_owned();
        self
    }

    pub fn set_team(mut self, team: &str) -> AzureDevopsClient {
        self.team = team.to_owned();
        self
    }

    pub async fn get(&self, query: String) -> Result<reqwest::Response, reqwest::Error> {
        let mut uri = PathBuf::new();
        uri.push("https://dev.azure.com/");
        uri.push(&self.organization);
        uri.push(&self.project);
        uri.push(&self.team);   // TODO: What about queries that don't have a team?
        uri.push("_apis");
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
