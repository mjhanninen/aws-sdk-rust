// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ErrorCode {
    AccessDenied,
    BadRequest,
    Conflict,
    Forbidden,
    NotFound,
    PhoneNumberAssociationsExist,
    PreconditionFailed,
    ResourceLimitExceeded,
    ServiceFailure,
    ServiceUnavailable,
    Throttled,
    Throttling,
    Unauthorized,
    Unprocessable,
    VoiceConnectorGroupAssociationsExist,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ErrorCode {
    fn from(s: &str) -> Self {
        match s {
            "AccessDenied" => ErrorCode::AccessDenied,
            "BadRequest" => ErrorCode::BadRequest,
            "Conflict" => ErrorCode::Conflict,
            "Forbidden" => ErrorCode::Forbidden,
            "NotFound" => ErrorCode::NotFound,
            "PhoneNumberAssociationsExist" => ErrorCode::PhoneNumberAssociationsExist,
            "PreconditionFailed" => ErrorCode::PreconditionFailed,
            "ResourceLimitExceeded" => ErrorCode::ResourceLimitExceeded,
            "ServiceFailure" => ErrorCode::ServiceFailure,
            "ServiceUnavailable" => ErrorCode::ServiceUnavailable,
            "Throttled" => ErrorCode::Throttled,
            "Throttling" => ErrorCode::Throttling,
            "Unauthorized" => ErrorCode::Unauthorized,
            "Unprocessable" => ErrorCode::Unprocessable,
            "VoiceConnectorGroupAssociationsExist" => {
                ErrorCode::VoiceConnectorGroupAssociationsExist
            }
            other => ErrorCode::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ErrorCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ErrorCode::from(s))
    }
}
impl ErrorCode {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorCode::AccessDenied => "AccessDenied",
            ErrorCode::BadRequest => "BadRequest",
            ErrorCode::Conflict => "Conflict",
            ErrorCode::Forbidden => "Forbidden",
            ErrorCode::NotFound => "NotFound",
            ErrorCode::PhoneNumberAssociationsExist => "PhoneNumberAssociationsExist",
            ErrorCode::PreconditionFailed => "PreconditionFailed",
            ErrorCode::ResourceLimitExceeded => "ResourceLimitExceeded",
            ErrorCode::ServiceFailure => "ServiceFailure",
            ErrorCode::ServiceUnavailable => "ServiceUnavailable",
            ErrorCode::Throttled => "Throttled",
            ErrorCode::Throttling => "Throttling",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::Unprocessable => "Unprocessable",
            ErrorCode::VoiceConnectorGroupAssociationsExist => {
                "VoiceConnectorGroupAssociationsExist"
            }
            ErrorCode::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "AccessDenied",
            "BadRequest",
            "Conflict",
            "Forbidden",
            "NotFound",
            "PhoneNumberAssociationsExist",
            "PreconditionFailed",
            "ResourceLimitExceeded",
            "ServiceFailure",
            "ServiceUnavailable",
            "Throttled",
            "Throttling",
            "Unauthorized",
            "Unprocessable",
            "VoiceConnectorGroupAssociationsExist",
        ]
    }
}
impl AsRef<str> for ErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The details of the data-retention settings for an <code>AppInstance</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceRetentionSettings {
    /// <p>The length of time in days to retain the messages in a channel.</p>
    pub channel_retention_settings: std::option::Option<crate::model::ChannelRetentionSettings>,
}
impl std::fmt::Debug for AppInstanceRetentionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceRetentionSettings");
        formatter.field(
            "channel_retention_settings",
            &self.channel_retention_settings,
        );
        formatter.finish()
    }
}
/// See [`AppInstanceRetentionSettings`](crate::model::AppInstanceRetentionSettings)
pub mod app_instance_retention_settings {
    /// A builder for [`AppInstanceRetentionSettings`](crate::model::AppInstanceRetentionSettings)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) channel_retention_settings:
            std::option::Option<crate::model::ChannelRetentionSettings>,
    }
    impl Builder {
        /// <p>The length of time in days to retain the messages in a channel.</p>
        pub fn channel_retention_settings(
            mut self,
            input: crate::model::ChannelRetentionSettings,
        ) -> Self {
            self.channel_retention_settings = Some(input);
            self
        }
        pub fn set_channel_retention_settings(
            mut self,
            input: std::option::Option<crate::model::ChannelRetentionSettings>,
        ) -> Self {
            self.channel_retention_settings = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceRetentionSettings`](crate::model::AppInstanceRetentionSettings)
        pub fn build(self) -> crate::model::AppInstanceRetentionSettings {
            crate::model::AppInstanceRetentionSettings {
                channel_retention_settings: self.channel_retention_settings,
            }
        }
    }
}
impl AppInstanceRetentionSettings {
    /// Creates a new builder-style object to manufacture [`AppInstanceRetentionSettings`](crate::model::AppInstanceRetentionSettings)
    pub fn builder() -> crate::model::app_instance_retention_settings::Builder {
        crate::model::app_instance_retention_settings::Builder::default()
    }
}

/// <p>The details of the retention settings for a channel.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ChannelRetentionSettings {
    /// <p>The time in days to retain the messages in a channel.</p>
    pub retention_days: std::option::Option<i32>,
}
impl std::fmt::Debug for ChannelRetentionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ChannelRetentionSettings");
        formatter.field("retention_days", &self.retention_days);
        formatter.finish()
    }
}
/// See [`ChannelRetentionSettings`](crate::model::ChannelRetentionSettings)
pub mod channel_retention_settings {
    /// A builder for [`ChannelRetentionSettings`](crate::model::ChannelRetentionSettings)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) retention_days: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The time in days to retain the messages in a channel.</p>
        pub fn retention_days(mut self, input: i32) -> Self {
            self.retention_days = Some(input);
            self
        }
        pub fn set_retention_days(mut self, input: std::option::Option<i32>) -> Self {
            self.retention_days = input;
            self
        }
        /// Consumes the builder and constructs a [`ChannelRetentionSettings`](crate::model::ChannelRetentionSettings)
        pub fn build(self) -> crate::model::ChannelRetentionSettings {
            crate::model::ChannelRetentionSettings {
                retention_days: self.retention_days,
            }
        }
    }
}
impl ChannelRetentionSettings {
    /// Creates a new builder-style object to manufacture [`ChannelRetentionSettings`](crate::model::ChannelRetentionSettings)
    pub fn builder() -> crate::model::channel_retention_settings::Builder {
        crate::model::channel_retention_settings::Builder::default()
    }
}

/// <p>Summary of the details of an <code>AppInstanceUser</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceUserSummary {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub app_instance_user_arn: std::option::Option<std::string::String>,
    /// <p>The name of an <code>AppInstanceUser</code>.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    pub metadata: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AppInstanceUserSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUserSummary");
        formatter.field("app_instance_user_arn", &self.app_instance_user_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`AppInstanceUserSummary`](crate::model::AppInstanceUserSummary)
pub mod app_instance_user_summary {
    /// A builder for [`AppInstanceUserSummary`](crate::model::AppInstanceUserSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) app_instance_user_arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) metadata: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
        pub fn app_instance_user_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.app_instance_user_arn = Some(input.into());
            self
        }
        pub fn set_app_instance_user_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.app_instance_user_arn = input;
            self
        }
        /// <p>The name of an <code>AppInstanceUser</code>.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
        pub fn metadata(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata = Some(input.into());
            self
        }
        pub fn set_metadata(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metadata = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceUserSummary`](crate::model::AppInstanceUserSummary)
        pub fn build(self) -> crate::model::AppInstanceUserSummary {
            crate::model::AppInstanceUserSummary {
                app_instance_user_arn: self.app_instance_user_arn,
                name: self.name,
                metadata: self.metadata,
            }
        }
    }
}
impl AppInstanceUserSummary {
    /// Creates a new builder-style object to manufacture [`AppInstanceUserSummary`](crate::model::AppInstanceUserSummary)
    pub fn builder() -> crate::model::app_instance_user_summary::Builder {
        crate::model::app_instance_user_summary::Builder::default()
    }
}

/// <p>Summary of the data for an <code>AppInstance</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceSummary {
    /// <p>The <code>AppInstance</code> ARN.</p>
    pub app_instance_arn: std::option::Option<std::string::String>,
    /// <p>The name of the <code>AppInstance</code>.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The metadata of the <code>AppInstance</code>.</p>
    pub metadata: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AppInstanceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceSummary");
        formatter.field("app_instance_arn", &self.app_instance_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`AppInstanceSummary`](crate::model::AppInstanceSummary)
pub mod app_instance_summary {
    /// A builder for [`AppInstanceSummary`](crate::model::AppInstanceSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) app_instance_arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) metadata: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The <code>AppInstance</code> ARN.</p>
        pub fn app_instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.app_instance_arn = Some(input.into());
            self
        }
        pub fn set_app_instance_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.app_instance_arn = input;
            self
        }
        /// <p>The name of the <code>AppInstance</code>.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The metadata of the <code>AppInstance</code>.</p>
        pub fn metadata(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata = Some(input.into());
            self
        }
        pub fn set_metadata(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metadata = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceSummary`](crate::model::AppInstanceSummary)
        pub fn build(self) -> crate::model::AppInstanceSummary {
            crate::model::AppInstanceSummary {
                app_instance_arn: self.app_instance_arn,
                name: self.name,
                metadata: self.metadata,
            }
        }
    }
}
impl AppInstanceSummary {
    /// Creates a new builder-style object to manufacture [`AppInstanceSummary`](crate::model::AppInstanceSummary)
    pub fn builder() -> crate::model::app_instance_summary::Builder {
        crate::model::app_instance_summary::Builder::default()
    }
}

/// <p>Summary of the details of an <code>AppInstanceAdmin</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceAdminSummary {
    /// <p>The details of the <code>AppInstanceAdmin</code>.</p>
    pub admin: std::option::Option<crate::model::Identity>,
}
impl std::fmt::Debug for AppInstanceAdminSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceAdminSummary");
        formatter.field("admin", &self.admin);
        formatter.finish()
    }
}
/// See [`AppInstanceAdminSummary`](crate::model::AppInstanceAdminSummary)
pub mod app_instance_admin_summary {
    /// A builder for [`AppInstanceAdminSummary`](crate::model::AppInstanceAdminSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) admin: std::option::Option<crate::model::Identity>,
    }
    impl Builder {
        /// <p>The details of the <code>AppInstanceAdmin</code>.</p>
        pub fn admin(mut self, input: crate::model::Identity) -> Self {
            self.admin = Some(input);
            self
        }
        pub fn set_admin(mut self, input: std::option::Option<crate::model::Identity>) -> Self {
            self.admin = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceAdminSummary`](crate::model::AppInstanceAdminSummary)
        pub fn build(self) -> crate::model::AppInstanceAdminSummary {
            crate::model::AppInstanceAdminSummary { admin: self.admin }
        }
    }
}
impl AppInstanceAdminSummary {
    /// Creates a new builder-style object to manufacture [`AppInstanceAdminSummary`](crate::model::AppInstanceAdminSummary)
    pub fn builder() -> crate::model::app_instance_admin_summary::Builder {
        crate::model::app_instance_admin_summary::Builder::default()
    }
}

/// <p>The details of a user.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Identity {
    /// <p>The ARN in an Identity.</p>
    pub arn: std::option::Option<std::string::String>,
    /// <p>The name in an Identity.</p>
    pub name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Identity");
        formatter.field("arn", &self.arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`Identity`](crate::model::Identity)
pub mod identity {
    /// A builder for [`Identity`](crate::model::Identity)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN in an Identity.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// <p>The name in an Identity.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Consumes the builder and constructs a [`Identity`](crate::model::Identity)
        pub fn build(self) -> crate::model::Identity {
            crate::model::Identity {
                arn: self.arn,
                name: self.name,
            }
        }
    }
}
impl Identity {
    /// Creates a new builder-style object to manufacture [`Identity`](crate::model::Identity)
    pub fn builder() -> crate::model::identity::Builder {
        crate::model::identity::Builder::default()
    }
}

/// <p>The details of an <code>AppInstanceUser</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceUser {
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub app_instance_user_arn: std::option::Option<std::string::String>,
    /// <p>The name of the <code>AppInstanceUser</code>.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
    pub metadata: std::option::Option<std::string::String>,
    /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
    pub created_timestamp: std::option::Option<smithy_types::Instant>,
    /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
    pub last_updated_timestamp: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for AppInstanceUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceUser");
        formatter.field("app_instance_user_arn", &self.app_instance_user_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.finish()
    }
}
/// See [`AppInstanceUser`](crate::model::AppInstanceUser)
pub mod app_instance_user {
    /// A builder for [`AppInstanceUser`](crate::model::AppInstanceUser)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) app_instance_user_arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) metadata: std::option::Option<std::string::String>,
        pub(crate) created_timestamp: std::option::Option<smithy_types::Instant>,
        pub(crate) last_updated_timestamp: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
        pub fn app_instance_user_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.app_instance_user_arn = Some(input.into());
            self
        }
        pub fn set_app_instance_user_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.app_instance_user_arn = input;
            self
        }
        /// <p>The name of the <code>AppInstanceUser</code>.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The metadata of the <code>AppInstanceUser</code>.</p>
        pub fn metadata(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata = Some(input.into());
            self
        }
        pub fn set_metadata(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metadata = input;
            self
        }
        /// <p>The time at which the <code>AppInstanceUser</code> was created.</p>
        pub fn created_timestamp(mut self, input: smithy_types::Instant) -> Self {
            self.created_timestamp = Some(input);
            self
        }
        pub fn set_created_timestamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.created_timestamp = input;
            self
        }
        /// <p>The time at which the <code>AppInstanceUser</code> was last updated.</p>
        pub fn last_updated_timestamp(mut self, input: smithy_types::Instant) -> Self {
            self.last_updated_timestamp = Some(input);
            self
        }
        pub fn set_last_updated_timestamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_updated_timestamp = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceUser`](crate::model::AppInstanceUser)
        pub fn build(self) -> crate::model::AppInstanceUser {
            crate::model::AppInstanceUser {
                app_instance_user_arn: self.app_instance_user_arn,
                name: self.name,
                metadata: self.metadata,
                created_timestamp: self.created_timestamp,
                last_updated_timestamp: self.last_updated_timestamp,
            }
        }
    }
}
impl AppInstanceUser {
    /// Creates a new builder-style object to manufacture [`AppInstanceUser`](crate::model::AppInstanceUser)
    pub fn builder() -> crate::model::app_instance_user::Builder {
        crate::model::app_instance_user::Builder::default()
    }
}

/// <p>The details of an <code>AppInstanceAdmin</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstanceAdmin {
    /// <p>The <code>AppInstanceAdmin</code> data.</p>
    pub admin: std::option::Option<crate::model::Identity>,
    /// <p>The ARN of the <code>AppInstance</code> for which the user is an administrator.</p>
    pub app_instance_arn: std::option::Option<std::string::String>,
    /// <p>The time at which an administrator was created.</p>
    pub created_timestamp: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for AppInstanceAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstanceAdmin");
        formatter.field("admin", &self.admin);
        formatter.field("app_instance_arn", &self.app_instance_arn);
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.finish()
    }
}
/// See [`AppInstanceAdmin`](crate::model::AppInstanceAdmin)
pub mod app_instance_admin {
    /// A builder for [`AppInstanceAdmin`](crate::model::AppInstanceAdmin)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) admin: std::option::Option<crate::model::Identity>,
        pub(crate) app_instance_arn: std::option::Option<std::string::String>,
        pub(crate) created_timestamp: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The <code>AppInstanceAdmin</code> data.</p>
        pub fn admin(mut self, input: crate::model::Identity) -> Self {
            self.admin = Some(input);
            self
        }
        pub fn set_admin(mut self, input: std::option::Option<crate::model::Identity>) -> Self {
            self.admin = input;
            self
        }
        /// <p>The ARN of the <code>AppInstance</code> for which the user is an administrator.</p>
        pub fn app_instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.app_instance_arn = Some(input.into());
            self
        }
        pub fn set_app_instance_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.app_instance_arn = input;
            self
        }
        /// <p>The time at which an administrator was created.</p>
        pub fn created_timestamp(mut self, input: smithy_types::Instant) -> Self {
            self.created_timestamp = Some(input);
            self
        }
        pub fn set_created_timestamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.created_timestamp = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstanceAdmin`](crate::model::AppInstanceAdmin)
        pub fn build(self) -> crate::model::AppInstanceAdmin {
            crate::model::AppInstanceAdmin {
                admin: self.admin,
                app_instance_arn: self.app_instance_arn,
                created_timestamp: self.created_timestamp,
            }
        }
    }
}
impl AppInstanceAdmin {
    /// Creates a new builder-style object to manufacture [`AppInstanceAdmin`](crate::model::AppInstanceAdmin)
    pub fn builder() -> crate::model::app_instance_admin::Builder {
        crate::model::app_instance_admin::Builder::default()
    }
}

/// <p>The details of an <code>AppInstance</code>, an instance of an Amazon Chime SDK messaging
/// application.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AppInstance {
    /// <p>The ARN of the messaging instance.</p>
    pub app_instance_arn: std::option::Option<std::string::String>,
    /// <p>The name of an <code>AppInstance</code>.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The time at which an <code>AppInstance</code> was created. In epoch milliseconds.</p>
    pub created_timestamp: std::option::Option<smithy_types::Instant>,
    /// <p>The time an <code>AppInstance</code> was last updated. In epoch milliseconds.</p>
    pub last_updated_timestamp: std::option::Option<smithy_types::Instant>,
    /// <p>The metadata of an <code>AppInstance</code>.</p>
    pub metadata: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AppInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AppInstance");
        formatter.field("app_instance_arn", &self.app_instance_arn);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("created_timestamp", &self.created_timestamp);
        formatter.field("last_updated_timestamp", &self.last_updated_timestamp);
        formatter.field("metadata", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`AppInstance`](crate::model::AppInstance)
pub mod app_instance {
    /// A builder for [`AppInstance`](crate::model::AppInstance)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) app_instance_arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) created_timestamp: std::option::Option<smithy_types::Instant>,
        pub(crate) last_updated_timestamp: std::option::Option<smithy_types::Instant>,
        pub(crate) metadata: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ARN of the messaging instance.</p>
        pub fn app_instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.app_instance_arn = Some(input.into());
            self
        }
        pub fn set_app_instance_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.app_instance_arn = input;
            self
        }
        /// <p>The name of an <code>AppInstance</code>.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The time at which an <code>AppInstance</code> was created. In epoch milliseconds.</p>
        pub fn created_timestamp(mut self, input: smithy_types::Instant) -> Self {
            self.created_timestamp = Some(input);
            self
        }
        pub fn set_created_timestamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.created_timestamp = input;
            self
        }
        /// <p>The time an <code>AppInstance</code> was last updated. In epoch milliseconds.</p>
        pub fn last_updated_timestamp(mut self, input: smithy_types::Instant) -> Self {
            self.last_updated_timestamp = Some(input);
            self
        }
        pub fn set_last_updated_timestamp(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_updated_timestamp = input;
            self
        }
        /// <p>The metadata of an <code>AppInstance</code>.</p>
        pub fn metadata(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata = Some(input.into());
            self
        }
        pub fn set_metadata(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.metadata = input;
            self
        }
        /// Consumes the builder and constructs a [`AppInstance`](crate::model::AppInstance)
        pub fn build(self) -> crate::model::AppInstance {
            crate::model::AppInstance {
                app_instance_arn: self.app_instance_arn,
                name: self.name,
                created_timestamp: self.created_timestamp,
                last_updated_timestamp: self.last_updated_timestamp,
                metadata: self.metadata,
            }
        }
    }
}
impl AppInstance {
    /// Creates a new builder-style object to manufacture [`AppInstance`](crate::model::AppInstance)
    pub fn builder() -> crate::model::app_instance::Builder {
        crate::model::app_instance::Builder::default()
    }
}

/// <p>Describes a tag applied to a resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The value of the tag.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.field("value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The key of the tag.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The value of the tag.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}
