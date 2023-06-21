use super::err::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use sha1::Digest;
use strfmt::strfmt;

use super::session::SessionInfo;
use crate::err::JellyfinError;
use crate::JellyfinClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "PrimaryImageTag")]
    pub primary_image_tag: String,
    #[serde(rename = "HasPassword")]
    pub has_password: bool,
    #[serde(rename = "HasConfiguredPassword")]
    pub has_configured_password: bool,
    #[serde(rename = "HasConfiguredEasyPassword")]
    pub has_configured_easy_password: bool,
    #[serde(rename = "EnableAutoLogin")]
    pub enable_auto_login: bool,
    #[serde(rename = "LastLoginDate")]
    pub last_login_date: String,
    #[serde(rename = "LastActivityDate")]
    pub last_activity_date: String,
    #[serde(rename = "Configuration")]
    pub configuration: UserConfiguration,
    #[serde(rename = "Policy")]
    pub policy: UserPolicy,
    #[serde(rename = "PrimaryImageAspectRatio")]
    pub primary_image_aspect_ratio: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConfiguration {
    #[serde(rename = "AudioLanguagePreference")]
    pub audio_language_preference: String,
    #[serde(rename = "PlayDefaultAudioTrack")]
    pub play_default_audio_track: bool,
    #[serde(rename = "SubtitleLanguagePreference")]
    pub subtitle_language_preference: String,
    #[serde(rename = "DisplayMissingEpisodes")]
    pub display_missing_episodes: bool,
    #[serde(rename = "GroupedFolders")]
    pub grouped_folders: Vec<String>,
    #[serde(rename = "SubtitleMode")]
    pub subtitle_mode: String,
    #[serde(rename = "DisplayCollectionsView")]
    pub display_collections_view: bool,
    #[serde(rename = "EnableLocalPassword")]
    pub enable_local_password: bool,
    #[serde(rename = "OrderedViews")]
    pub ordered_views: Vec<String>,
    #[serde(rename = "LatestItemsExcludes")]
    pub latest_items_excludes: Vec<String>,
    #[serde(rename = "MyMediaExcludes")]
    pub my_media_excludes: Vec<String>,
    #[serde(rename = "HidePlayedInLatest")]
    pub hide_played_in_latest: bool,
    #[serde(rename = "RememberAudioSelections")]
    pub remember_audio_selections: bool,
    #[serde(rename = "RememberSubtitleSelections")]
    pub remember_subtitle_selections: bool,
    #[serde(rename = "EnableNextEpisodeAutoPlay")]
    pub enable_next_episode_auto_play: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPolicy {
    #[serde(rename = "IsAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "IsDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "MaxParentalRating")]
    pub max_parental_rating: i64,
    #[serde(rename = "BlockedTags")]
    pub blocked_tags: Vec<String>,
    #[serde(rename = "EnableUserPreferenceAccess")]
    pub enable_user_preference_access: bool,
    #[serde(rename = "AccessSchedules")]
    pub access_schedules: Vec<UserAccessSchedule>,
    #[serde(rename = "BlockUnratedItems")]
    pub block_unrated_items: Vec<String>,
    #[serde(rename = "EnableRemoteControlOfOtherUsers")]
    pub enable_remote_control_of_other_users: bool,
    #[serde(rename = "EnableSharedDeviceControl")]
    pub enable_shared_device_control: bool,
    #[serde(rename = "EnableRemoteAccess")]
    pub enable_remote_access: bool,
    #[serde(rename = "EnableLiveTvManagement")]
    pub enable_live_tv_management: bool,
    #[serde(rename = "EnableLiveTvAccess")]
    pub enable_live_tv_access: bool,
    #[serde(rename = "EnableMediaPlayback")]
    pub enable_media_playback: bool,
    #[serde(rename = "EnableAudioPlaybackTranscoding")]
    pub enable_audio_playback_transcoding: bool,
    #[serde(rename = "EnableVideoPlaybackTranscoding")]
    pub enable_video_playback_transcoding: bool,
    #[serde(rename = "EnablePlaybackRemuxing")]
    pub enable_playback_remuxing: bool,
    #[serde(rename = "ForceRemoteSourceTranscoding")]
    pub force_remote_source_transcoding: bool,
    #[serde(rename = "EnableContentDeletion")]
    pub enable_content_deletion: bool,
    #[serde(rename = "EnableContentDeletionFromFolders")]
    pub enable_content_deletion_from_folders: Vec<String>,
    #[serde(rename = "EnableContentDownloading")]
    pub enable_content_downloading: bool,
    #[serde(rename = "EnableSyncTranscoding")]
    pub enable_sync_transcoding: bool,
    #[serde(rename = "EnableMediaConversion")]
    pub enable_media_conversion: bool,
    #[serde(rename = "EnabledDevices")]
    pub enabled_devices: Vec<String>,
    #[serde(rename = "EnableAllDevices")]
    pub enable_all_devices: bool,
    #[serde(rename = "EnabledChannels")]
    pub enabled_channels: Vec<String>,
    #[serde(rename = "EnableAllChannels")]
    pub enable_all_channels: bool,
    #[serde(rename = "EnabledFolders")]
    pub enabled_folders: Vec<String>,
    #[serde(rename = "EnableAllFolders")]
    pub enable_all_folders: bool,
    #[serde(rename = "InvalidLoginAttemptCount")]
    pub invalid_login_attempt_count: i64,
    #[serde(rename = "LoginAttemptsBeforeLockout")]
    pub login_attempts_before_lockout: i64,
    #[serde(rename = "MaxActiveSessions")]
    pub max_active_sessions: i64,
    #[serde(rename = "EnablePublicSharing")]
    pub enable_public_sharing: bool,
    #[serde(rename = "BlockedMediaFolders")]
    pub blocked_media_folders: Vec<String>,
    #[serde(rename = "BlockedChannels")]
    pub blocked_channels: Vec<String>,
    #[serde(rename = "RemoteClientBitrateLimit")]
    pub remote_client_bitrate_limit: i64,
    #[serde(rename = "AuthenticationProviderId")]
    pub authentication_provider_id: String,
    #[serde(rename = "PasswordResetProviderId")]
    pub password_reset_provider_id: String,
    #[serde(rename = "SyncPlayAccess")]
    pub sync_play_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccessSchedule {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: String,
    #[serde(rename = "StartHour")]
    pub start_hour: i64,
    #[serde(rename = "EndHour")]
    pub end_hour: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    #[serde(rename = "User")]
    pub user: User,
    #[serde(rename = "SessionInfo")]
    pub session_info: SessionInfo,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
}

impl UserAuth {
    pub fn to_emby_header(&self) -> String {
        let device_name = whoami::devicename().replace(" ", "_");
        format!("Emby UserId=\"{}\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"{}\"", self.user.id, device_name, md5::compute(device_name.clone()), self.access_token)
    }
}

static GET_USERS_EP: &str = "/Users";
static GET_USER_BY_ID_EP: &str = "/Users/{id}";
static DELETE_USER_EP: &str = "/Users/{id}";
static UPDATE_USER_EP: &str = "/Users/{id}";
static AUTH_USER_STD_EP: &str = "/Users/{id}/Authenticate";
static UPDATE_USER_CONF_EP: &str = "/Users/{id}/Configutation";
static UPDATE_USER_PASSWORD_EP: &str = "/Users/{id}/Password";
static UPDATE_USER_POLICY_EP: &str = "/Users/{id}/Policy";
static AUTH_USER_NAME_EP: &str = "/Users/AuthenticateByName";
static USER_FORGOT_PASSWORD_EP: &str = "/Users/ForgotPassword";
static USER_REDEEM_FORGOT_PASSWORD_PIN_EP: &str = "/Users/ForgotPassword/Pin";
static GET_USER_BY_AUTH_EP: &str = "/Users/Me";
static CREATE_USER_EP: &str = "/Users/New";
static GET_PUBLIC_USER_LIST_EP: &str = "/Users/Public";

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

//TODO: use phantom data to make auth better
//TODO: add docs
impl JellyfinClient {
    /// Gets a list of all users that the `UserAuth` has access to, given some filters.
    pub async fn get_users(&self, is_hidden: bool, is_disabled: bool) -> Result<Vec<User>> {
        let mut req = surf::get(&self.url.join(GET_USERS_EP)?)
            .query(&GetUsersQuery {
                is_hidden,
                is_disabled,
            })?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(req.body_json::<Vec<User>>().await?)
    }

    pub async fn get_user_by_id<T: Into<String>>(&self, id: T) -> Result<User> {
        let mut req = surf::get(&self.url.join(&strfmt!(GET_USER_BY_ID_EP, id=>id.into())?)?)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(req.body_json::<User>().await?)
    }

    pub async fn delete_user<T: Into<String>>(&self, id: T) -> Result<()> {
        let _req = surf::delete(&self.url.join(&strfmt!(DELETE_USER_EP, id=>id.into())?)?)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(())
    }

    pub async fn update_user<T: Into<String>>(&self, id: T, new_info: User) -> Result<()> {
        let _req = surf::post(&self.url.join(&strfmt!(UPDATE_USER_EP, id=>id.into())?)?)
            .body_json(&new_info)?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(())
    }

    pub async fn auth_user_std<T: Into<String> + Clone>(&mut self, id: T, password: T) -> Result<()> {
        let mut hasher = sha1::Sha1::new();
        hasher.update(password.clone().into());
        let device_name = whoami::devicename().replace(" ", "_");

        let mut req = surf::post(&self.url.join(&strfmt!(AUTH_USER_STD_EP, id=>id.into())?)?)
            .query(&AuthUserStdQuery {
                pw: password.into(),
                password: format!("{:x}", hasher.finalize())
            })?
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .await?;
        
        self.auth = Some(req.body_json::<UserAuth>().await?);
        Ok(())
    }

    pub async fn update_user_conf<T: Into<String>>(&self, id: T, new_conf: UserConfiguration) -> Result<()> {
        let _req = surf::post(&self.url.join(&strfmt!(UPDATE_USER_CONF_EP, id=>id.into())?)?)
            .body_json(&new_conf)?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(())
    }

    pub async fn update_user_password<T: Into<String>>(&self, id: T, new_password: T) -> Result<()> {
        let _req = surf::post(&self.url.join(&strfmt!(UPDATE_USER_PASSWORD_EP, id=>id.into())?)?)
            .body_json(&json!({ "NewPw": new_password.into() }))?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(())
    }

    pub async fn update_user_policy<T: Into<String>>(&self, id: T, new_policy: UserPolicy) -> Result<()> {
        let _req = surf::post(&self.url.join(&strfmt!(UPDATE_USER_POLICY_EP, id=>id.into())?)?)
            .body_json(&new_policy)?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(())
    }

    pub async fn auth_user_name<T: Into<String>>(&mut self, username: T, password: T) -> Result<()> {
        let device_name = whoami::devicename().replace(" ", "_");

        let mut req = surf::post(&self.url.join(&AUTH_USER_NAME_EP)?)
            .body_json(&json!({
                "Username": username.into(),
                "Pw": password.into()
            }))?
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .await?;

        self.auth = Some(req.body_json::<UserAuth>().await?);
        Ok(())
    }

    pub async fn user_forgot_password<T: Into<String>>(&self, username: T) -> Result<()> {
        let device_name = whoami::devicename().replace(" ", "_");

        let _req = surf::post(&self.url.join(&USER_FORGOT_PASSWORD_EP)?)
            .body_json(&json!({
                "EnteredUsername": username.into()
            }))?
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .await?;
        Ok(())
    }

    pub async fn user_redeem_forgot_password_pin<T: Into<String>>(&self, pin: T) -> Result<()> {
        let device_name = whoami::devicename().replace(" ", "_");

        let _req = surf::post(&self.url.join(&USER_REDEEM_FORGOT_PASSWORD_PIN_EP)?)
            .body_json(&json!({
                "Pin": pin.into()
            }))?
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .await?;
        Ok(())
    }

    pub async fn get_user_by_auth(&self) -> Result<User> {
        let mut req = surf::get(&self.url.join(&GET_USER_BY_AUTH_EP)?)
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(req.body_json::<User>().await?)
    }

    pub async fn create_user<T: Into<String>>(&self, username: T, password: T) -> Result<User> {
        let mut req = surf::post(&self.url.join(&CREATE_USER_EP)?)
            .body_json(&json!({
                "Name": username.into(),
                "Password": password.into()
            }))?
            .header(
                "X-Emby-Authorization",
                self.auth
                    .as_ref()
                    .ok_or(JellyfinError::AuthNotFound)?
                    .to_emby_header(),
            )
            .await?;
        Ok(req.body_json::<User>().await?)
    }

    pub async fn get_public_user_list(&self) -> Result<Vec<User>> {
        let device_name = whoami::devicename().replace(" ", "_");

        let mut req = surf::get(&self.url.join(&GET_PUBLIC_USER_LIST_EP)?)
            .header("X-Emby-Authorization", format!("Emby UserId=\"\", Client=\"jellyfin-rs\", Device=\"{}\", DeviceId=\"{:x}\", Version=1, Token=\"\"", device_name, md5::compute(device_name.clone())))
            .await?;
        Ok(req.body_json::<Vec<User>>().await?)
    }
}
