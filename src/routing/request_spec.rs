use derive_builder::Builder;
use http::Request;

#[derive(Debug, Clone)]
pub enum PathSegment {
    Literal(String),
    Label,
    Greedy,
}

#[derive(Debug, Clone)]
pub enum QuerySegment {
    Key(String),
    KeyValue(String, String),
}

#[derive(Debug, Clone)]
pub enum HostPrefixSegment {
    Literal(String),
    Label,
}

#[derive(Debug, Clone, Builder)]
pub struct UriSpec {
    host_prefix: Option<Vec<HostPrefixSegment>>,
    path_segments: Vec<PathSegment>,
    query_segments: Option<Vec<QuerySegment>>,
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    method: http::Method,
    uri_spec: UriSpec,
}

#[derive(Debug)]
pub enum Match {
    Yes,
    No,
    MethodNotAllowed,
}

impl RequestSpec {
    pub fn new(method: http::Method, uri_spec: UriSpec) -> Self {
        RequestSpec { method, uri_spec }
    }

    pub(super) fn matches<B>(&self, req: &Request<B>) -> Match {
        Match::Yes
    }

    pub fn always_get() -> Self {
        RequestSpec {
            method: http::Method::GET,
            uri_spec: UriSpecBuilder::default().build().unwrap(),
        }
    }
}
