use super::err::Result;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::err::JellyfinError;
use crate::JellyfinClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityLogEntry {
    pub id: u32,
    pub name: String,
    pub overview: Option<String>,
    pub short_overview: Option<String>,
    pub r#type: String,
    pub item_id: Option<String>,
    pub date: String,
    pub user_id: String,
    pub severity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityLogEntries {
    pub items: Vec<ActivityLogEntry>,
    pub total_record_count: u32,
    pub start_index: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetActivityLogEntriesQuery {
    start_index: Option<u32>,
    limit: Option<u32>,
    min_date: Option<String>,
    has_user_id: bool,
}

impl JellyfinClient {
    pub async fn get_activity_log_entries(
        &self,
        start_index: Option<u32>,
        limit: Option<u32>,
        min_date: Option<String>,
        has_user_id: bool,
    ) -> Result<ActivityLogEntries> {
        let req = self
            .client
            .get(format!("{}System/ActivityLog/Entries", self.url,))
            .query(&GetActivityLogEntriesQuery {
                start_index,
                limit,
                min_date,
                has_user_id,
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
}
