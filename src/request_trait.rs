use std::marker::PhantomData;

struct Query {
    name: String,
    value: String,
}

pub struct RequestSuper<T> {
    organization: String,
    project: String,
    team: String,   // TODO: make into a list, need to then make multiple queries
    resource_path: String,
    // api version?
    queries: Vec<Query>,
    phantom: PhantomData<T>,
}

impl<T> RequestSuper<T> {
    pub fn new(resource_path: &str) -> RequestSuper<T> {
        RequestSuper {
            resource_path: resource_path.to_owned(),
            organization: String::from(""),
            project: String::from(""),
            team: String::from(""),
            queries: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn set_organization(mut self, organization: &str) -> RequestSuper<T> {
        self.organization = organization.to_owned();
        self
    }

    pub fn set_project(mut self, project: &str) -> RequestSuper<T> {
        self.project = project.to_owned();
        self
    }

    pub fn set_team(mut self, team: &str) -> RequestSuper<T> {
        self.team = team.to_owned();
        self
    }

    pub fn add_query(mut self, name: &str, value: &str) -> RequestSuper<T> {
        self.queries.push(Query {
            name: name.to_owned(),
            value: value.to_owned(),
        });
        self
    }

    // TODO - set api version?
}