use std::collections::BTreeMap;

use tide::{Endpoint, convert::Serialize};

pub fn test<State: Clone + Send + Sync + 'static>(endpoint: impl Endpoint<State>) -> TideUnitBuilder<State> {
    TideUnitBuilder { endpoint: Box::new(endpoint), state: None, _params: BTreeMap::new(), _query: BTreeMap::new() }
}

pub struct TideUnitBuilder<State> {
    endpoint: Box<dyn Endpoint<State>>,
    state: Option<State>,

    _params: BTreeMap<String, String>,
    _query: BTreeMap<String, String>,
}

impl<State: Clone + Send + Sync + 'static> TideUnitBuilder<State> {
    /// Set the state for the endpoint under test
    pub fn with_state(mut self, state: State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn param(self, _param: &str, _value: &str) -> Self {
        self
    }

    /// Add the query string
    pub fn with_query<T: Serialize>(self, _query: T) -> Self {
        self
    }

    pub fn ext<T>(self, _ext: T) -> Self {
        self
    }

    pub fn test_endpoint(&self, _endpoint: impl Endpoint<State>) {

    }
}

#[cfg(test)]
mod when_testing_an_endpoint {
    use super::*;
    use tide::{Request, Result};
    use serde::{Serialize};

    async fn endpoint(_request: Request<()>) -> Result {
        Ok("result".into())
    }

    #[derive(Serialize)]
    struct Query {
        value1: u32,
        value2: String,
    }

    #[test]
    fn it_works() {
        test(endpoint)
            .param("param1", "value1").param("param2", "value2")
            .with_query(Query {value1: 3, value2: "test".to_string()});
    }
}
