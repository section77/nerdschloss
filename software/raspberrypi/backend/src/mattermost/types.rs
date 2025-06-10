pub mod users {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UserResponse {
        pub id: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
        pub username: String,
        #[serde(rename = "first_name")]
        pub first_name: String,
        #[serde(rename = "last_name")]
        pub last_name: String,
        pub nickname: String,
        pub email: String,
        #[serde(rename = "email_verified")]
        pub email_verified: bool,
        #[serde(rename = "auth_service")]
        pub auth_service: String,
        pub roles: String,
        pub locale: String,
        #[serde(rename = "notify_props")]
        pub notify_props: NotifyProps,
        pub props: Props,
        #[serde(rename = "last_password_update")]
        pub last_password_update: i64,
        #[serde(rename = "last_picture_update")]
        pub last_picture_update: i64,
        #[serde(rename = "failed_attempts")]
        pub failed_attempts: i64,
        #[serde(rename = "mfa_active")]
        pub mfa_active: bool,
        pub timezone: Timezone,
        #[serde(rename = "terms_of_service_id")]
        pub terms_of_service_id: String,
        #[serde(rename = "terms_of_service_create_at")]
        pub terms_of_service_create_at: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotifyProps {
        pub email: String,
        pub push: String,
        pub desktop: String,
        #[serde(rename = "desktop_sound")]
        pub desktop_sound: String,
        #[serde(rename = "mention_keys")]
        pub mention_keys: String,
        pub channel: String,
        #[serde(rename = "first_name")]
        pub first_name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Props {}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Timezone {
        pub use_automatic_timezone: bool,
        pub manual_timezone: String,
        pub automatic_timezone: String,
    }
}

mod login {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginResponce {
        pub id: String,
        pub create_at: i64,
        pub update_at: i64,
        pub delete_at: i64,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub nickname: String,
        pub email: String,
        pub email_verified: bool,
        pub auth_service: String,
        pub roles: String,
        pub locale: String,
        pub notify_props: NotifyProps,
        pub props: Props,
        pub last_password_update: i64,
        pub last_picture_update: i64,
        pub failed_attempts: i64,
        pub mfa_active: bool,
        pub timezone: Timezone,
        pub terms_of_service_id: String,
        pub terms_of_service_create_at: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotifyProps {
        pub email: String,
        pub push: String,
        pub desktop: String,
        pub desktop_sound: String,
        pub mention_keys: String,
        pub channel: String,
        pub first_name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Props {}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Timezone {
        pub use_automatic_timezone: bool,
        pub manual_timezone: String,
        pub automatic_timezone: String,
    }
}

pub mod teams {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TeamResponse {
        pub id: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
        #[serde(rename = "display_name")]
        pub display_name: String,
        pub name: String,
        pub description: String,
        pub email: String,
        pub r#type: String,
        #[serde(rename = "allowed_domains")]
        pub allowed_domains: String,
        #[serde(rename = "invite_id")]
        pub invite_id: String,
        #[serde(rename = "allow_open_invite")]
        pub allow_open_invite: bool,
        #[serde(rename = "policy_id")]
        pub policy_id: String,
    }
}

pub mod posts {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PostResponse {
        pub id: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
        #[serde(rename = "edit_at")]
        pub edit_at: i64,
        #[serde(rename = "user_id")]
        pub user_id: String,
        #[serde(rename = "channel_id")]
        pub channel_id: String,
        #[serde(rename = "root_id")]
        pub root_id: String,
        #[serde(rename = "original_id")]
        pub original_id: String,
        pub message: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub props: Props,
        pub hashtag: String,
        #[serde(rename = "file_ids")]
        pub file_ids: Vec<String>,
        #[serde(rename = "pending_post_id")]
        pub pending_post_id: String,
        pub metadata: Metadata,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Props {}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Metadata {
        pub embeds: Vec<Embed>,
        pub emojis: Vec<Emoji>,
        pub files: Vec<File>,
        pub images: Images,
        pub reactions: Vec<Reaction>,
        pub priority: Priority,
        pub acknowledgements: Vec<Acknowledgement>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Embed {
        #[serde(rename = "type")]
        pub type_field: String,
        pub url: String,
        pub data: Data,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Emoji {
        pub id: String,
        #[serde(rename = "creator_id")]
        pub creator_id: String,
        pub name: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct File {
        pub id: String,
        #[serde(rename = "user_id")]
        pub user_id: String,
        #[serde(rename = "post_id")]
        pub post_id: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
        pub name: String,
        pub extension: String,
        pub size: i64,
        #[serde(rename = "mime_type")]
        pub mime_type: String,
        pub width: i64,
        pub height: i64,
        #[serde(rename = "has_preview_image")]
        pub has_preview_image: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images {}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Reaction {
        #[serde(rename = "user_id")]
        pub user_id: String,
        #[serde(rename = "post_id")]
        pub post_id: String,
        #[serde(rename = "emoji_name")]
        pub emoji_name: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Priority {
        pub priority: String,
        #[serde(rename = "requested_ack")]
        pub requested_ack: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Acknowledgement {
        #[serde(rename = "user_id")]
        pub user_id: String,
        #[serde(rename = "post_id")]
        pub post_id: String,
        #[serde(rename = "acknowledged_at")]
        pub acknowledged_at: i64,
    }
}

pub mod channels {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ChannelPatchRequest {
        pub name: String,
        #[serde(rename = "display_name")]
        pub display_name: String,
        pub purpose: String,
        pub header: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ChannelResponse {
        pub id: String,
        #[serde(rename = "create_at")]
        pub create_at: i64,
        #[serde(rename = "update_at")]
        pub update_at: i64,
        #[serde(rename = "delete_at")]
        pub delete_at: i64,
        #[serde(rename = "team_id")]
        pub team_id: String,
        #[serde(rename = "type")]
        pub type_field: String,
        #[serde(rename = "display_name")]
        pub display_name: String,
        pub name: String,
        pub header: String,
        pub purpose: String,
        #[serde(rename = "last_post_at")]
        pub last_post_at: i64,
        #[serde(rename = "total_msg_count")]
        pub total_msg_count: i64,
        #[serde(rename = "extra_update_at")]
        pub extra_update_at: i64,
        #[serde(rename = "creator_id")]
        pub creator_id: String,
    }
}
