// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>A home region control is an object that specifies the home region for an account, with
/// some additional information. It contains a target (always of type <code>ACCOUNT</code>), an
/// ID, and a time at which the home region was set.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct HomeRegionControl {
    /// <p>A unique identifier that's generated for each home region control. It's always a string
    /// that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
    pub control_id: std::option::Option<std::string::String>,
    /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1"
    /// are valid home regions.</p>
    pub home_region: std::option::Option<std::string::String>,
    /// <p>The target parameter specifies the identifier to which the home region is applied, which
    /// is always an <code>ACCOUNT</code>. It applies the home region to the current
    /// <code>ACCOUNT</code>.</p>
    pub target: std::option::Option<crate::model::Target>,
    /// <p>A timestamp representing the time when the customer called
    /// <code>CreateHomeregionControl</code> and set the home region for the account.</p>
    pub requested_time: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for HomeRegionControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("HomeRegionControl");
        formatter.field("control_id", &self.control_id);
        formatter.field("home_region", &self.home_region);
        formatter.field("target", &self.target);
        formatter.field("requested_time", &self.requested_time);
        formatter.finish()
    }
}
/// See [`HomeRegionControl`](crate::model::HomeRegionControl)
pub mod home_region_control {
    /// A builder for [`HomeRegionControl`](crate::model::HomeRegionControl)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_id: std::option::Option<std::string::String>,
        pub(crate) home_region: std::option::Option<std::string::String>,
        pub(crate) target: std::option::Option<crate::model::Target>,
        pub(crate) requested_time: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>A unique identifier that's generated for each home region control. It's always a string
        /// that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
        pub fn control_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_id = Some(input.into());
            self
        }
        pub fn set_control_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.control_id = input;
            self
        }
        /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1"
        /// are valid home regions.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.home_region = Some(input.into());
            self
        }
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.home_region = input;
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which
        /// is always an <code>ACCOUNT</code>. It applies the home region to the current
        /// <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.target = Some(input);
            self
        }
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.target = input;
            self
        }
        /// <p>A timestamp representing the time when the customer called
        /// <code>CreateHomeregionControl</code> and set the home region for the account.</p>
        pub fn requested_time(mut self, input: smithy_types::Instant) -> Self {
            self.requested_time = Some(input);
            self
        }
        pub fn set_requested_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.requested_time = input;
            self
        }
        /// Consumes the builder and constructs a [`HomeRegionControl`](crate::model::HomeRegionControl)
        pub fn build(self) -> crate::model::HomeRegionControl {
            crate::model::HomeRegionControl {
                control_id: self.control_id,
                home_region: self.home_region,
                target: self.target,
                requested_time: self.requested_time,
            }
        }
    }
}
impl HomeRegionControl {
    /// Creates a new builder-style object to manufacture [`HomeRegionControl`](crate::model::HomeRegionControl)
    pub fn builder() -> crate::model::home_region_control::Builder {
        crate::model::home_region_control::Builder::default()
    }
}

/// <p>The target parameter specifies the identifier to which the home region is applied, which
/// is always an <code>ACCOUNT</code>. It applies the home region to the current
/// <code>ACCOUNT</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Target {
    /// <p>The target type is always an <code>ACCOUNT</code>.</p>
    pub r#type: std::option::Option<crate::model::TargetType>,
    /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for
    /// which the control was created. (This must be the current account.) </p>
    pub id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Target");
        formatter.field("r#type", &self.r#type);
        formatter.field("id", &self.id);
        formatter.finish()
    }
}
/// See [`Target`](crate::model::Target)
pub mod target {
    /// A builder for [`Target`](crate::model::Target)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) r#type: std::option::Option<crate::model::TargetType>,
        pub(crate) id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The target type is always an <code>ACCOUNT</code>.</p>
        pub fn r#type(mut self, input: crate::model::TargetType) -> Self {
            self.r#type = Some(input);
            self
        }
        pub fn set_type(mut self, input: std::option::Option<crate::model::TargetType>) -> Self {
            self.r#type = input;
            self
        }
        /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for
        /// which the control was created. (This must be the current account.) </p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// Consumes the builder and constructs a [`Target`](crate::model::Target)
        pub fn build(self) -> crate::model::Target {
            crate::model::Target {
                r#type: self.r#type,
                id: self.id,
            }
        }
    }
}
impl Target {
    /// Creates a new builder-style object to manufacture [`Target`](crate::model::Target)
    pub fn builder() -> crate::model::target::Builder {
        crate::model::target::Builder::default()
    }
}

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
pub enum TargetType {
    Account,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for TargetType {
    fn from(s: &str) -> Self {
        match s {
            "ACCOUNT" => TargetType::Account,
            other => TargetType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for TargetType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TargetType::from(s))
    }
}
impl TargetType {
    pub fn as_str(&self) -> &str {
        match self {
            TargetType::Account => "ACCOUNT",
            TargetType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["ACCOUNT"]
    }
}
impl AsRef<str> for TargetType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
