use url::Url;

mod user;
mod err;
mod session;
mod items;

#[derive(Debug, Clone, PartialEq)]
pub struct JellyfinConnection {
    url: Url
}

impl JellyfinConnection {
    /// Creates a new `JellyfinConnection`
    /// * `url` The base jellyfin server url, without a traling "/"
    pub fn new<T: Into<Url>>(url: T) -> Self {
        Self {
            url: url.into()
        }
    }
}