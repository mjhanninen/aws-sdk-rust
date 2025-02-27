// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>
/// Result structure used for requests to updated project configuration.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateProjectOutput {
    /// <p>
    /// Detailed information about the updated AWS Mobile Hub project.
    /// </p>
    pub details: std::option::Option<crate::model::ProjectDetails>,
}
impl std::fmt::Debug for UpdateProjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateProjectOutput");
        formatter.field("details", &self.details);
        formatter.finish()
    }
}
/// See [`UpdateProjectOutput`](crate::output::UpdateProjectOutput)
pub mod update_project_output {
    /// A builder for [`UpdateProjectOutput`](crate::output::UpdateProjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) details: std::option::Option<crate::model::ProjectDetails>,
    }
    impl Builder {
        /// <p>
        /// Detailed information about the updated AWS Mobile Hub project.
        /// </p>
        pub fn details(mut self, input: crate::model::ProjectDetails) -> Self {
            self.details = Some(input);
            self
        }
        pub fn set_details(
            mut self,
            input: std::option::Option<crate::model::ProjectDetails>,
        ) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateProjectOutput`](crate::output::UpdateProjectOutput)
        pub fn build(self) -> crate::output::UpdateProjectOutput {
            crate::output::UpdateProjectOutput {
                details: self.details,
            }
        }
    }
}
impl UpdateProjectOutput {
    /// Creates a new builder-style object to manufacture [`UpdateProjectOutput`](crate::output::UpdateProjectOutput)
    pub fn builder() -> crate::output::update_project_output::Builder {
        crate::output::update_project_output::Builder::default()
    }
}

/// <p>
/// Result structure used for requests to list projects in AWS Mobile Hub.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListProjectsOutput {
    /// <p>
    /// List of projects.
    /// </p>
    pub projects: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
    /// <p>
    /// Pagination token. Set to null to start listing records from start.
    /// If non-null pagination token is returned in a result, then pass its
    /// value in here in another request to list more entries.
    /// </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListProjectsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListProjectsOutput");
        formatter.field("projects", &self.projects);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListProjectsOutput`](crate::output::ListProjectsOutput)
pub mod list_projects_output {
    /// A builder for [`ListProjectsOutput`](crate::output::ListProjectsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) projects: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn projects(mut self, input: impl Into<crate::model::ProjectSummary>) -> Self {
            let mut v = self.projects.unwrap_or_default();
            v.push(input.into());
            self.projects = Some(v);
            self
        }
        pub fn set_projects(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
        ) -> Self {
            self.projects = input;
            self
        }
        /// <p>
        /// Pagination token. Set to null to start listing records from start.
        /// If non-null pagination token is returned in a result, then pass its
        /// value in here in another request to list more entries.
        /// </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListProjectsOutput`](crate::output::ListProjectsOutput)
        pub fn build(self) -> crate::output::ListProjectsOutput {
            crate::output::ListProjectsOutput {
                projects: self.projects,
                next_token: self.next_token,
            }
        }
    }
}
impl ListProjectsOutput {
    /// Creates a new builder-style object to manufacture [`ListProjectsOutput`](crate::output::ListProjectsOutput)
    pub fn builder() -> crate::output::list_projects_output::Builder {
        crate::output::list_projects_output::Builder::default()
    }
}

/// <p>
/// Result structure contains a list of all available bundles with details.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListBundlesOutput {
    /// <p>
    /// A list of bundles.
    /// </p>
    pub bundle_list: std::option::Option<std::vec::Vec<crate::model::BundleDetails>>,
    /// <p>
    /// Pagination token. If non-null pagination token is returned in a result,
    /// then pass its value in another request to fetch more entries.
    /// </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListBundlesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListBundlesOutput");
        formatter.field("bundle_list", &self.bundle_list);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListBundlesOutput`](crate::output::ListBundlesOutput)
pub mod list_bundles_output {
    /// A builder for [`ListBundlesOutput`](crate::output::ListBundlesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) bundle_list: std::option::Option<std::vec::Vec<crate::model::BundleDetails>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn bundle_list(mut self, input: impl Into<crate::model::BundleDetails>) -> Self {
            let mut v = self.bundle_list.unwrap_or_default();
            v.push(input.into());
            self.bundle_list = Some(v);
            self
        }
        pub fn set_bundle_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BundleDetails>>,
        ) -> Self {
            self.bundle_list = input;
            self
        }
        /// <p>
        /// Pagination token. If non-null pagination token is returned in a result,
        /// then pass its value in another request to fetch more entries.
        /// </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListBundlesOutput`](crate::output::ListBundlesOutput)
        pub fn build(self) -> crate::output::ListBundlesOutput {
            crate::output::ListBundlesOutput {
                bundle_list: self.bundle_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListBundlesOutput {
    /// Creates a new builder-style object to manufacture [`ListBundlesOutput`](crate::output::ListBundlesOutput)
    pub fn builder() -> crate::output::list_bundles_output::Builder {
        crate::output::list_bundles_output::Builder::default()
    }
}

/// <p>
/// Result structure used for requests to export project configuration details.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExportProjectOutput {
    /// <p>
    /// URL which can be used to download the exported project configuation file(s).
    /// </p>
    pub download_url: std::option::Option<std::string::String>,
    /// <p>
    /// URL which can be shared to allow other AWS users to create their own project
    /// in AWS Mobile Hub with the same configuration as the specified project. This
    /// URL pertains to a snapshot in time of the project configuration that is created
    /// when this API is called. If you want to share additional changes to your project
    /// configuration, then you will need to create and share a new snapshot by calling
    /// this method again.
    /// </p>
    pub share_url: std::option::Option<std::string::String>,
    /// <p>
    /// Unique identifier for the exported snapshot of the project configuration. This
    /// snapshot identifier is included in the share URL.
    /// </p>
    pub snapshot_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ExportProjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExportProjectOutput");
        formatter.field("download_url", &self.download_url);
        formatter.field("share_url", &self.share_url);
        formatter.field("snapshot_id", &self.snapshot_id);
        formatter.finish()
    }
}
/// See [`ExportProjectOutput`](crate::output::ExportProjectOutput)
pub mod export_project_output {
    /// A builder for [`ExportProjectOutput`](crate::output::ExportProjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) download_url: std::option::Option<std::string::String>,
        pub(crate) share_url: std::option::Option<std::string::String>,
        pub(crate) snapshot_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>
        /// URL which can be used to download the exported project configuation file(s).
        /// </p>
        pub fn download_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.download_url = Some(input.into());
            self
        }
        pub fn set_download_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.download_url = input;
            self
        }
        /// <p>
        /// URL which can be shared to allow other AWS users to create their own project
        /// in AWS Mobile Hub with the same configuration as the specified project. This
        /// URL pertains to a snapshot in time of the project configuration that is created
        /// when this API is called. If you want to share additional changes to your project
        /// configuration, then you will need to create and share a new snapshot by calling
        /// this method again.
        /// </p>
        pub fn share_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.share_url = Some(input.into());
            self
        }
        pub fn set_share_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.share_url = input;
            self
        }
        /// <p>
        /// Unique identifier for the exported snapshot of the project configuration. This
        /// snapshot identifier is included in the share URL.
        /// </p>
        pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.snapshot_id = Some(input.into());
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.snapshot_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ExportProjectOutput`](crate::output::ExportProjectOutput)
        pub fn build(self) -> crate::output::ExportProjectOutput {
            crate::output::ExportProjectOutput {
                download_url: self.download_url,
                share_url: self.share_url,
                snapshot_id: self.snapshot_id,
            }
        }
    }
}
impl ExportProjectOutput {
    /// Creates a new builder-style object to manufacture [`ExportProjectOutput`](crate::output::ExportProjectOutput)
    pub fn builder() -> crate::output::export_project_output::Builder {
        crate::output::export_project_output::Builder::default()
    }
}

/// <p>
/// Result structure which contains link to download custom-generated SDK and
/// tool packages used to integrate mobile web or app clients with backed
/// AWS resources.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ExportBundleOutput {
    /// <p>
    /// URL which contains the custom-generated SDK and tool packages used
    /// to integrate the client mobile app or web app with the AWS resources
    /// created by the AWS Mobile Hub project.
    /// </p>
    pub download_url: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ExportBundleOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExportBundleOutput");
        formatter.field("download_url", &self.download_url);
        formatter.finish()
    }
}
/// See [`ExportBundleOutput`](crate::output::ExportBundleOutput)
pub mod export_bundle_output {
    /// A builder for [`ExportBundleOutput`](crate::output::ExportBundleOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) download_url: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>
        /// URL which contains the custom-generated SDK and tool packages used
        /// to integrate the client mobile app or web app with the AWS resources
        /// created by the AWS Mobile Hub project.
        /// </p>
        pub fn download_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.download_url = Some(input.into());
            self
        }
        pub fn set_download_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.download_url = input;
            self
        }
        /// Consumes the builder and constructs a [`ExportBundleOutput`](crate::output::ExportBundleOutput)
        pub fn build(self) -> crate::output::ExportBundleOutput {
            crate::output::ExportBundleOutput {
                download_url: self.download_url,
            }
        }
    }
}
impl ExportBundleOutput {
    /// Creates a new builder-style object to manufacture [`ExportBundleOutput`](crate::output::ExportBundleOutput)
    pub fn builder() -> crate::output::export_bundle_output::Builder {
        crate::output::export_bundle_output::Builder::default()
    }
}

/// <p>
/// Result structure used for requests of project details.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeProjectOutput {
    /// <p>
    /// Detailed information about an AWS Mobile Hub project.
    /// </p>
    pub details: std::option::Option<crate::model::ProjectDetails>,
}
impl std::fmt::Debug for DescribeProjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeProjectOutput");
        formatter.field("details", &self.details);
        formatter.finish()
    }
}
/// See [`DescribeProjectOutput`](crate::output::DescribeProjectOutput)
pub mod describe_project_output {
    /// A builder for [`DescribeProjectOutput`](crate::output::DescribeProjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) details: std::option::Option<crate::model::ProjectDetails>,
    }
    impl Builder {
        /// <p>
        /// Detailed information about an AWS Mobile Hub project.
        /// </p>
        pub fn details(mut self, input: crate::model::ProjectDetails) -> Self {
            self.details = Some(input);
            self
        }
        pub fn set_details(
            mut self,
            input: std::option::Option<crate::model::ProjectDetails>,
        ) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeProjectOutput`](crate::output::DescribeProjectOutput)
        pub fn build(self) -> crate::output::DescribeProjectOutput {
            crate::output::DescribeProjectOutput {
                details: self.details,
            }
        }
    }
}
impl DescribeProjectOutput {
    /// Creates a new builder-style object to manufacture [`DescribeProjectOutput`](crate::output::DescribeProjectOutput)
    pub fn builder() -> crate::output::describe_project_output::Builder {
        crate::output::describe_project_output::Builder::default()
    }
}

/// <p>
/// Result structure contains the details of the bundle.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeBundleOutput {
    /// <p>
    /// The details of the bundle.
    /// </p>
    pub details: std::option::Option<crate::model::BundleDetails>,
}
impl std::fmt::Debug for DescribeBundleOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeBundleOutput");
        formatter.field("details", &self.details);
        formatter.finish()
    }
}
/// See [`DescribeBundleOutput`](crate::output::DescribeBundleOutput)
pub mod describe_bundle_output {
    /// A builder for [`DescribeBundleOutput`](crate::output::DescribeBundleOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) details: std::option::Option<crate::model::BundleDetails>,
    }
    impl Builder {
        /// <p>
        /// The details of the bundle.
        /// </p>
        pub fn details(mut self, input: crate::model::BundleDetails) -> Self {
            self.details = Some(input);
            self
        }
        pub fn set_details(
            mut self,
            input: std::option::Option<crate::model::BundleDetails>,
        ) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeBundleOutput`](crate::output::DescribeBundleOutput)
        pub fn build(self) -> crate::output::DescribeBundleOutput {
            crate::output::DescribeBundleOutput {
                details: self.details,
            }
        }
    }
}
impl DescribeBundleOutput {
    /// Creates a new builder-style object to manufacture [`DescribeBundleOutput`](crate::output::DescribeBundleOutput)
    pub fn builder() -> crate::output::describe_bundle_output::Builder {
        crate::output::describe_bundle_output::Builder::default()
    }
}

/// <p>
/// Result structure used in response to request to delete a project.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteProjectOutput {
    /// <p>
    /// Resources which were deleted.
    /// </p>
    pub deleted_resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
    /// <p>
    /// Resources which were not deleted, due to a risk of losing potentially
    /// important data or files.
    /// </p>
    pub orphaned_resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
}
impl std::fmt::Debug for DeleteProjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteProjectOutput");
        formatter.field("deleted_resources", &self.deleted_resources);
        formatter.field("orphaned_resources", &self.orphaned_resources);
        formatter.finish()
    }
}
/// See [`DeleteProjectOutput`](crate::output::DeleteProjectOutput)
pub mod delete_project_output {
    /// A builder for [`DeleteProjectOutput`](crate::output::DeleteProjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) deleted_resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
        pub(crate) orphaned_resources: std::option::Option<std::vec::Vec<crate::model::Resource>>,
    }
    impl Builder {
        pub fn deleted_resources(mut self, input: impl Into<crate::model::Resource>) -> Self {
            let mut v = self.deleted_resources.unwrap_or_default();
            v.push(input.into());
            self.deleted_resources = Some(v);
            self
        }
        pub fn set_deleted_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Resource>>,
        ) -> Self {
            self.deleted_resources = input;
            self
        }
        pub fn orphaned_resources(mut self, input: impl Into<crate::model::Resource>) -> Self {
            let mut v = self.orphaned_resources.unwrap_or_default();
            v.push(input.into());
            self.orphaned_resources = Some(v);
            self
        }
        pub fn set_orphaned_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Resource>>,
        ) -> Self {
            self.orphaned_resources = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteProjectOutput`](crate::output::DeleteProjectOutput)
        pub fn build(self) -> crate::output::DeleteProjectOutput {
            crate::output::DeleteProjectOutput {
                deleted_resources: self.deleted_resources,
                orphaned_resources: self.orphaned_resources,
            }
        }
    }
}
impl DeleteProjectOutput {
    /// Creates a new builder-style object to manufacture [`DeleteProjectOutput`](crate::output::DeleteProjectOutput)
    pub fn builder() -> crate::output::delete_project_output::Builder {
        crate::output::delete_project_output::Builder::default()
    }
}

/// <p>
/// Result structure used in response to a request to create a project.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateProjectOutput {
    /// <p>
    /// Detailed information about the created AWS Mobile Hub project.
    /// </p>
    pub details: std::option::Option<crate::model::ProjectDetails>,
}
impl std::fmt::Debug for CreateProjectOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateProjectOutput");
        formatter.field("details", &self.details);
        formatter.finish()
    }
}
/// See [`CreateProjectOutput`](crate::output::CreateProjectOutput)
pub mod create_project_output {
    /// A builder for [`CreateProjectOutput`](crate::output::CreateProjectOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) details: std::option::Option<crate::model::ProjectDetails>,
    }
    impl Builder {
        /// <p>
        /// Detailed information about the created AWS Mobile Hub project.
        /// </p>
        pub fn details(mut self, input: crate::model::ProjectDetails) -> Self {
            self.details = Some(input);
            self
        }
        pub fn set_details(
            mut self,
            input: std::option::Option<crate::model::ProjectDetails>,
        ) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateProjectOutput`](crate::output::CreateProjectOutput)
        pub fn build(self) -> crate::output::CreateProjectOutput {
            crate::output::CreateProjectOutput {
                details: self.details,
            }
        }
    }
}
impl CreateProjectOutput {
    /// Creates a new builder-style object to manufacture [`CreateProjectOutput`](crate::output::CreateProjectOutput)
    pub fn builder() -> crate::output::create_project_output::Builder {
        crate::output::create_project_output::Builder::default()
    }
}
