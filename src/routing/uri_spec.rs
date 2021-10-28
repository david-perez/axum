#[derive(Clone)]
pub struct UriSpec;

impl UriSpec {
    /// Always matches.
    pub fn always() -> Self {
        UriSpec
    }
}
