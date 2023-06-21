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
    pub async fn new<T: Into<String>>(url: T) -> err::Result<Self> {
        Ok(Self {
            url: Url::parse(&url.into())?,
            auth: None
        })
    }

    /// Creates a new `JellyfinConnection` with auth
    /// * `url` The base jellyfin server url, without a traling "/"
    /// * `id` The Id of the user to auth with
    /// * `password` The plain text password of the user to auth with
    pub async fn new_auth_std<T: Into<String>>(url: T, id: T, password: T) -> err::Result<Self> {
        let mut client = Self {
            url: Url::parse(&url.into())?,
            auth: None
        };
        client.auth_user_std(id.into(), password.into()).await?;
        Ok(client)
    }

    /// Creates a new `JellyfinConnection` with auth
    /// * `url` The base jellyfin server url, without a traling "/"
    /// * `username` The username of the user to auth with
    /// * `password` The plain text password of the user to auth with
    pub async fn new_auth_name<T: Into<String>>(url: T, username: T, password: T) -> err::Result<Self> {
        let mut client = Self {
            url: Url::parse(&url.into())?,
            auth: None
        };
        client.auth_user_name(username.into(), password.into()).await?;
        Ok(client)
    }
}