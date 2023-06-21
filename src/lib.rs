use url::Url;
use user::UserAuth;

mod user;
mod err;
mod session;
mod items;

#[derive(Debug, Clone, PartialEq)]
pub struct JellyfinClient {
    url: Url,
    auth: Option<UserAuth>
}

impl JellyfinClient {
    /// Creates a new `JellyfinConnection`
    /// * `url` The base jellyfin server url, without a traling "/"
    pub fn new(url: String) -> err::Result<Self> {
        Ok(Self {
            url: Url::parse(&url)?,
            auth: None
        })
    }
}