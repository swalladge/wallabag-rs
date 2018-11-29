use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

use crate::utils::serde::parse_intbool;

use super::annotations::Annotations;
use super::common::ID;
use super::tags::Tags;

/// type alias: a list of entries as returned from some endpoints
pub type Entries = Vec<Entry>;

/// A struct representing an entry from wallabag (a full saved article including
/// all annotations and tags; annotations and tags do not need to be requested
/// separately).
#[derive(Deserialize, Debug)]
pub struct Entry {
    pub annotations: Option<Annotations>,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub domain_name: Option<String>,
    pub headers: Option<String>,
    pub http_status: Option<String>,
    pub id: ID,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_archived: bool,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_public: bool,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_starred: bool,
    pub language: Option<String>,
    pub mimetype: Option<String>,
    pub origin_url: Option<String>,
    pub preview_picture: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub published_by: Option<String>,

    /// estimated reading time in minutes (appears to be generated by the
    /// server)
    pub reading_time: u32,

    pub starred_at: Option<DateTime<Utc>>,
    pub tags: Tags,
    pub title: Option<String>,
    pub uid: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub url: Option<String>,
    pub user_email: String,
    pub user_id: ID,
    pub user_name: String,
}

/// A struct representing a deleted entry from wallabag (a full saved article including
/// annotations and tags). The only difference from the full entry is that this
/// doesn't have an id. Only used internally because a full entry gets
/// reconstituted before being returned to the client.
#[derive(Deserialize, Debug)]
pub(crate) struct DeletedEntry {
    pub annotations: Option<Annotations>,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub domain_name: Option<String>,
    pub headers: Option<String>,
    pub http_status: Option<String>,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_archived: bool,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_public: bool,

    #[serde(deserialize_with = "parse_intbool")]
    pub is_starred: bool,
    pub language: Option<String>,
    pub mimetype: Option<String>,
    pub origin_url: Option<String>,
    pub preview_picture: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub published_by: Option<String>,
    pub reading_time: u32,
    pub starred_at: Option<DateTime<Utc>>,
    pub tags: Tags,
    pub title: Option<String>,
    pub uid: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub url: Option<String>,
    pub user_email: String,
    pub user_id: ID,
    pub user_name: String,
}

/// This is implemented so that an Entry can be used interchangably with an ID
/// for some client methods. For convenience.
impl From<Entry> for ID {
    fn from(entry: Entry) -> Self {
        entry.id
    }
}

/// Internal struct for retrieving a list of entries from the api when
/// paginated.
#[derive(Deserialize, Debug)]
pub(crate) struct PaginatedEntries {
    pub limit: u32,
    pub page: u32,
    pub pages: u32,
    pub total: u32,
    pub _embedded: EmbeddedEntries,
}

/// Entries as stored in `PaginatedEntries`.
#[derive(Deserialize, Debug)]
pub(crate) struct EmbeddedEntries {
    pub items: Entries,
}
