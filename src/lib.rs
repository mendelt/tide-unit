use std::collections::BTreeMap;

use tide::{Endpoint, Response, convert::Serialize};

pub fn test<State: Clone + Send + Sync + 'static>(endpoint: impl Endpoint<State>) -> TideUnitBuilder<State> {
    TideUnitBuilder { _endpoint: Box::new(endpoint), _params: BTreeMap::new(), _query: BTreeMap::new() }
}

pub struct TideUnitBuilder<State> {
    _endpoint: Box<dyn Endpoint<State>>,

    _params: BTreeMap<String, String>,
    _query: BTreeMap<String, String>,
}

impl<State: Clone + Send + Sync + 'static> TideUnitBuilder<State> {
    pub fn param(self, _param: &str, _value: &str) -> Self {
        self
    }

    /// Add the query string
    pub fn with_query<T: Serialize>(self, _query: T) -> Self {
        self
    }

    /// Add request state extensions
    pub fn ext<T>(self, _ext: T) -> Self {
        self
    }

    /// Run this endpoint using the supplied state
    pub async fn run_with(self, _state: &State) -> Response {
        todo!();
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
