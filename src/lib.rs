use std::collections::{BTreeMap, HashMap};

use tide::{Endpoint, Response, convert::Serialize};

pub fn test<State: Clone + Send + Sync + 'static>(endpoint: impl Endpoint<State>) -> TideUnitBuilder<State> {
    TideUnitBuilder { endpoint: Box::new(endpoint), params: Params::new(), _query: BTreeMap::new() }
}

pub struct TideUnitBuilder<State> {
    endpoint: Box<dyn Endpoint<State>>,

    params: Params,
    _query: BTreeMap<String, String>,
}

impl<State: Clone + Send + Sync + 'static> TideUnitBuilder<State> {
    pub fn with_params(mut self, params: Params) -> Self {
        self.params = params;
        self
    }

    /// Add the query string
    pub fn with_query<T: Serialize>(mut self, _query: T) -> Self {
        self
    }

    /// Add request state extensions
    pub fn ext<T>(self, _ext: T) -> Self {
        self
    }

    /// Run this endpoint using the supplied state
    pub async fn run_with(self, state: State) -> Response {
        let mut server = tide::Server::with_state(state);

        server.at("/").get(self.endpoint);

        let client = surf::client();


        todo!();
    }
}

/// Parameters for insertion in paths
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Params(HashMap<String, String>);

impl Params {
    /// Create new params
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a parameter
    pub fn insert<P: ToString, V: ToString>(&mut self, param: P, value: V) {
        self.0.insert(param.to_string(), value.to_string());
    }
}

/// Construct parameters for the request
#[macro_export]
macro_rules! params {
    () => {{
        Params::new()
    }};

    ($( $param:expr => $value:expr ),+ ) => {{
        let mut pm: Params = Params::new();
        $(pm.insert($param.to_string(), $value);)*
        pm
    }};
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
            .with_params(params!("param1" => "value1", "param2" => "value2"))
            .with_query(Query {value1: 3, value2: "test".to_string()});
    }
}
