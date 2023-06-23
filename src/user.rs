use super::err::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use sha1::Digest;

use super::session::SessionInfo;
use crate::err::JellyfinError;
use crate::JellyfinClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: String,
    pub server_id: String,
    pub server_name: Option<String>,
    pub id: String,
    pub primary_image_tag: Option<String>,
    pub has_password: bool,
    pub has_configured_password: bool,
    pub has_configured_easy_password: bool,
    pub enable_auto_login: bool,
    pub last_login_date: Option<String>,
    pub last_activity_date: Option<String>,
    pub configuration: UserConfiguration,
    pub policy: UserPolicy,
    pub primary_image_aspect_ratio: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserConfiguration {
    pub audio_language_preference: Option<String>,
    pub play_default_audio_track: bool,
    pub subtitle_language_preference: String,
    pub display_missing_episodes: bool,
    pub grouped_folders: Vec<String>,
    pub subtitle_mode: String,
    pub display_collections_view: bool,
    pub enable_local_password: bool,
    pub ordered_views: Vec<String>,
    pub latest_items_excludes: Vec<String>,
    pub my_media_excludes: Vec<String>,
    pub hide_played_in_latest: bool,
    pub remember_audio_selections: bool,
    pub remember_subtitle_selections: bool,
    pub enable_next_episode_auto_play: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPolicy {
    pub is_administrator: bool,
    pub is_hidden: bool,
    pub is_disabled: bool,
    pub max_parental_rating: Option<i64>,
    pub blocked_tags: Vec<String>,
    pub enable_user_preference_access: bool,
    pub access_schedules: Vec<UserAccessSchedule>,
    pub block_unrated_items: Vec<String>,
    pub enable_remote_control_of_other_users: bool,
    pub enable_shared_device_control: bool,
    pub enable_remote_access: bool,
    pub enable_live_tv_management: bool,
    pub enable_live_tv_access: bool,
    pub enable_media_playback: bool,
    pub enable_audio_playback_transcoding: bool,
    pub enable_video_playback_transcoding: bool,
    pub enable_playback_remuxing: bool,
    pub force_remote_source_transcoding: bool,
    pub enable_content_deletion: bool,
    pub enable_content_deletion_from_folders: Vec<String>,
    pub enable_content_downloading: bool,
    pub enable_sync_transcoding: bool,
    pub enable_media_conversion: bool,
    pub enabled_devices: Vec<String>,
    pub enable_all_devices: bool,
    pub enabled_channels: Vec<String>,
    pub enable_all_channels: bool,
    pub enabled_folders: Vec<String>,
    pub enable_all_folders: bool,
    pub invalid_login_attempt_count: i64,
    pub login_attempts_before_lockout: i64,
    pub max_active_sessions: i64,
    pub enable_public_sharing: bool,
    pub blocked_media_folders: Vec<String>,
    pub blocked_channels: Vec<String>,
    pub remote_client_bitrate_limit: i64,
    pub authentication_provider_id: String,
    pub password_reset_provider_id: String,
    pub sync_play_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAccessSchedule {
    pub user_id: String,
    pub day_of_week: String,
    pub start_hour: i64,
    pub end_hour: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAuth {
    pub user: User,
    pub session_info: SessionInfo,
    pub access_token: String,
    pub server_id: String,
}

impl UserAuth {
    pub fn to_emby_header(&self) -> String {
        let device_name = whoami::devicename().replace(' ', "_");
        format!("Emby UserId=\"{}\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"{}\"", self.user.id, device_name, md5::compute(device_name.clone()), self.access_token)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GetUsersQuery {
    is_hidden: bool,
    is_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AuthUserStdQuery {
    pw: String,
    password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AuthUserNameQuery {
    username: String,
    pw: String,
}

//TODO: use phantom data to make auth better
//TODO: add docs
impl JellyfinClient {
    /// Gets a list of all users that the `UserAuth` has access to, given some filters.
    pub async fn get_users(&self, is_hidden: bool, is_disabled: bool) -> Result<Vec<User>> {
        let req = self
            .client
            .get(format!(
                "{}/Users",
                self.url,
            ))
            .query(&GetUsersQuery {
                is_hidden,
                is_disabled,
            })
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(req.json().await?)
    }

    pub async fn get_user_by_id<T: Into<String>>(&self, id: T) -> Result<User> {
        let req = self
            .client
            .get(format!(
                "{}/Users/{}",
                self.url,
                id.into().as_str()
            ))
                .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(req.json().await?)
    }

    pub async fn delete_user<T: Into<String>>(&self, id: T) -> Result<()> {
        let _req = self
            .client
            .delete(format!(
                "{}/Users/{}",
                self.url,
                id.into().as_str()
            ))
                .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn update_user<T: Into<String>>(&self, id: T, new_info: User) -> Result<()> {
        let _req = self
            .client
            .post(format!(
                "{}/Users/{}",
                self.url,
                id.into().as_str()
            ))
            .json(&new_info)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn auth_user_std<T: Into<String> + Clone>(
        &mut self,
        id: T,
        password: T,
    ) -> Result<()> {
        let mut hasher = sha1::Sha1::new();
        hasher.update(password.clone().into());
        let device_name = whoami::devicename().replace(' ', "_");

        let req = self
            .client.post(format!(
                "{}/Users/{}/Authenticate",
                self.url,
                id.into().as_str()
            ))
            .query(&AuthUserStdQuery {
                pw: password.into(),
                password: format!("{:x}", hasher.finalize())
            }).header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await?;

        self.auth = Some(req.json().await?);
        Ok(())
    }

    pub async fn update_user_conf<T: Into<String>>(
        &self,
        id: T,
        new_conf: UserConfiguration,
    ) -> Result<()> {
        let _req = self
            .client
            .post(
                format!(
                    "{}/Users/{}/Configuration",
                    self.url,
                    id.into().as_str()
                )
            )
            .json(&new_conf)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn update_user_password<T: Into<String>>(
        &self,
        id: T,
        new_password: T,
    ) -> Result<()> {
        let _req = self
            .client
            .post(
                format!(
                    "{}/Users/{}/Password",
                    self.url,
                    id.into().as_str()
                )
            )
            .json(&json!({ "NewPw": new_password.into() }))
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn update_user_policy<T: Into<String>>(
        &self,
        id: T,
        new_policy: UserPolicy,
    ) -> Result<()> {
        let _req = self
            .client
            .post(
                format!(
                    "{}/Users/{}/Policy",
                    self.url,
                    id.into().as_str()
                )
            )
            .json(&new_policy)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn auth_user_name<T: Into<String>>(
        &mut self,
        username: T,
        password: T,
    ) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");

        let req = self.client.post(format!(
            "{}/Users/AuthenticateByName",
            self.url
        ))
            .json(&AuthUserNameQuery {
                username: username.into(),
                pw: password.into()
            })
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await?;

        self.auth = Some(req.json().await?);
        Ok(())
    }

    pub async fn user_forgot_password<T: Into<String>>(&self, username: T) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");

        let _req = self.client.post(format!(
            "{}/Users/ForgotPassword",
            self.url
        )).json(&json!({
                "EnteredUsername": username.into()
            }))
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await?;

        Ok(())
    }

    pub async fn user_redeem_forgot_password_pin<T: Into<String>>(&self, pin: T) -> Result<()> {
        let device_name = whoami::devicename().replace(' ', "_");

        let _req = self.client.post(format!(
            "{}/Users/ForgotPassword/Pin",
            self.url
        )).json(&json!({
                "Pin": pin.into()
            }))
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_user_by_auth(&self) -> Result<User> {
        let req = self
            .client
            .get(format!(
                "{}/Users/Me",
                self.url
            ))
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(req.json().await?)
    }

    pub async fn create_user<T: Into<String>>(&self, username: T, password: T) -> Result<User> {
        let req = self
            .client
            .post(format!(
                "{}/Users/New",
                self.url
            ))
            .json(&json!({
                "Name": username.into(),
                "Password": password.into()
            }))
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .send()
            .await?;

        Ok(req.json().await?)
    }

    pub async fn get_public_user_list(&self) -> Result<Vec<User>> {
        let device_name = whoami::devicename().replace(' ', "_");

        let req = self.client.get(format!(
            "{}/Users/Public",
            self.url
        ))
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .send()
            .await?;

        Ok(req.json().await?)
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {

    }
}
