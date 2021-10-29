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
        let mut request_path_segments_iter = req.uri().path().split("/").filter(|s| !s.is_empty());
        let request_path_idx = 0;

        for path_segment in self.uri_spec.path_segments.iter() {
            match request_path_segments_iter.next() {
                // There are more path segments in the request spec, but we have reached the end of
                // the request path.
                None => return Match::No,
                Some(request_path) => match path_segment {
                    PathSegment::Literal(literal) => {
                        if literal != request_path {
                            return Match::No;
                        }
                    }
                    PathSegment::Label => continue,
                    PathSegment::Greedy => todo!(),
                },
            }
        }

        Match::Yes
    }

    pub fn always_get() -> Self {
        RequestSpec {
            method: http::Method::GET,
            uri_spec: UriSpecBuilder::default().build().unwrap(),
        }
    }
}
