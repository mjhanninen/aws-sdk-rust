// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutRecommendationFeedbackOutput {}
impl std::fmt::Debug for PutRecommendationFeedbackOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutRecommendationFeedbackOutput");
        formatter.finish()
    }
}
/// See [`PutRecommendationFeedbackOutput`](crate::output::PutRecommendationFeedbackOutput)
pub mod put_recommendation_feedback_output {
    /// A builder for [`PutRecommendationFeedbackOutput`](crate::output::PutRecommendationFeedbackOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutRecommendationFeedbackOutput`](crate::output::PutRecommendationFeedbackOutput)
        pub fn build(self) -> crate::output::PutRecommendationFeedbackOutput {
            crate::output::PutRecommendationFeedbackOutput {}
        }
    }
}
impl PutRecommendationFeedbackOutput {
    /// Creates a new builder-style object to manufacture [`PutRecommendationFeedbackOutput`](crate::output::PutRecommendationFeedbackOutput)
    pub fn builder() -> crate::output::put_recommendation_feedback_output::Builder {
        crate::output::put_recommendation_feedback_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>
    /// An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:
    /// </p>
    /// <ul>
    /// <li>
    /// <p>A <i>tag key</i> (for example, <code>CostCenter</code>,
    /// <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
    /// keys are case sensitive.</p>
    /// </li>
    /// <li>
    /// <p>An optional field known as a <i>tag value</i> (for example,
    /// <code>111122223333</code>, <code>Production</code>, or a team name).
    /// Omitting the tag value is the same as using an empty string. Like tag keys, tag
    /// values are case sensitive.</p>
    /// </li>
    /// </ul>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListRepositoryAssociationsOutput {
    /// <p>A list of repository associations that meet the criteria of the request.</p>
    pub repository_association_summaries:
        std::option::Option<std::vec::Vec<crate::model::RepositoryAssociationSummary>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRecommendations</code> request.
    /// When the results of a <code>ListRecommendations</code> request exceed <code>maxResults</code>, this
    /// value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more
    /// results to return. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListRepositoryAssociationsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListRepositoryAssociationsOutput");
        formatter.field(
            "repository_association_summaries",
            &self.repository_association_summaries,
        );
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListRepositoryAssociationsOutput`](crate::output::ListRepositoryAssociationsOutput)
pub mod list_repository_associations_output {
    /// A builder for [`ListRepositoryAssociationsOutput`](crate::output::ListRepositoryAssociationsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) repository_association_summaries:
            std::option::Option<std::vec::Vec<crate::model::RepositoryAssociationSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn repository_association_summaries(
            mut self,
            input: impl Into<crate::model::RepositoryAssociationSummary>,
        ) -> Self {
            let mut v = self.repository_association_summaries.unwrap_or_default();
            v.push(input.into());
            self.repository_association_summaries = Some(v);
            self
        }
        pub fn set_repository_association_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::RepositoryAssociationSummary>>,
        ) -> Self {
            self.repository_association_summaries = input;
            self
        }
        /// <p>The <code>nextToken</code> value to include in a future <code>ListRecommendations</code> request.
        /// When the results of a <code>ListRecommendations</code> request exceed <code>maxResults</code>, this
        /// value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more
        /// results to return. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListRepositoryAssociationsOutput`](crate::output::ListRepositoryAssociationsOutput)
        pub fn build(self) -> crate::output::ListRepositoryAssociationsOutput {
            crate::output::ListRepositoryAssociationsOutput {
                repository_association_summaries: self.repository_association_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListRepositoryAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`ListRepositoryAssociationsOutput`](crate::output::ListRepositoryAssociationsOutput)
    pub fn builder() -> crate::output::list_repository_associations_output::Builder {
        crate::output::list_repository_associations_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListRecommendationsOutput {
    /// <p>
    /// List of recommendations for the requested code review.
    /// </p>
    pub recommendation_summaries:
        std::option::Option<std::vec::Vec<crate::model::RecommendationSummary>>,
    /// <p>
    /// Pagination token.
    /// </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListRecommendationsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListRecommendationsOutput");
        formatter.field("recommendation_summaries", &self.recommendation_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListRecommendationsOutput`](crate::output::ListRecommendationsOutput)
pub mod list_recommendations_output {
    /// A builder for [`ListRecommendationsOutput`](crate::output::ListRecommendationsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) recommendation_summaries:
            std::option::Option<std::vec::Vec<crate::model::RecommendationSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn recommendation_summaries(
            mut self,
            input: impl Into<crate::model::RecommendationSummary>,
        ) -> Self {
            let mut v = self.recommendation_summaries.unwrap_or_default();
            v.push(input.into());
            self.recommendation_summaries = Some(v);
            self
        }
        pub fn set_recommendation_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::RecommendationSummary>>,
        ) -> Self {
            self.recommendation_summaries = input;
            self
        }
        /// <p>
        /// Pagination token.
        /// </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListRecommendationsOutput`](crate::output::ListRecommendationsOutput)
        pub fn build(self) -> crate::output::ListRecommendationsOutput {
            crate::output::ListRecommendationsOutput {
                recommendation_summaries: self.recommendation_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListRecommendationsOutput {
    /// Creates a new builder-style object to manufacture [`ListRecommendationsOutput`](crate::output::ListRecommendationsOutput)
    pub fn builder() -> crate::output::list_recommendations_output::Builder {
        crate::output::list_recommendations_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListRecommendationFeedbackOutput {
    /// <p> Recommendation feedback summaries corresponding to the code review ARN. </p>
    pub recommendation_feedback_summaries:
        std::option::Option<std::vec::Vec<crate::model::RecommendationFeedbackSummary>>,
    /// <p>
    /// If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page.
    /// Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged.
    /// </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListRecommendationFeedbackOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListRecommendationFeedbackOutput");
        formatter.field(
            "recommendation_feedback_summaries",
            &self.recommendation_feedback_summaries,
        );
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListRecommendationFeedbackOutput`](crate::output::ListRecommendationFeedbackOutput)
pub mod list_recommendation_feedback_output {
    /// A builder for [`ListRecommendationFeedbackOutput`](crate::output::ListRecommendationFeedbackOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) recommendation_feedback_summaries:
            std::option::Option<std::vec::Vec<crate::model::RecommendationFeedbackSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn recommendation_feedback_summaries(
            mut self,
            input: impl Into<crate::model::RecommendationFeedbackSummary>,
        ) -> Self {
            let mut v = self.recommendation_feedback_summaries.unwrap_or_default();
            v.push(input.into());
            self.recommendation_feedback_summaries = Some(v);
            self
        }
        pub fn set_recommendation_feedback_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::RecommendationFeedbackSummary>>,
        ) -> Self {
            self.recommendation_feedback_summaries = input;
            self
        }
        /// <p>
        /// If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page.
        /// Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged.
        /// </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListRecommendationFeedbackOutput`](crate::output::ListRecommendationFeedbackOutput)
        pub fn build(self) -> crate::output::ListRecommendationFeedbackOutput {
            crate::output::ListRecommendationFeedbackOutput {
                recommendation_feedback_summaries: self.recommendation_feedback_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListRecommendationFeedbackOutput {
    /// Creates a new builder-style object to manufacture [`ListRecommendationFeedbackOutput`](crate::output::ListRecommendationFeedbackOutput)
    pub fn builder() -> crate::output::list_recommendation_feedback_output::Builder {
        crate::output::list_recommendation_feedback_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListCodeReviewsOutput {
    /// <p>
    /// A list of code reviews that meet the criteria of the request.
    /// </p>
    pub code_review_summaries: std::option::Option<std::vec::Vec<crate::model::CodeReviewSummary>>,
    /// <p>
    /// Pagination token.
    /// </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListCodeReviewsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListCodeReviewsOutput");
        formatter.field("code_review_summaries", &self.code_review_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListCodeReviewsOutput`](crate::output::ListCodeReviewsOutput)
pub mod list_code_reviews_output {
    /// A builder for [`ListCodeReviewsOutput`](crate::output::ListCodeReviewsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) code_review_summaries:
            std::option::Option<std::vec::Vec<crate::model::CodeReviewSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn code_review_summaries(
            mut self,
            input: impl Into<crate::model::CodeReviewSummary>,
        ) -> Self {
            let mut v = self.code_review_summaries.unwrap_or_default();
            v.push(input.into());
            self.code_review_summaries = Some(v);
            self
        }
        pub fn set_code_review_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::CodeReviewSummary>>,
        ) -> Self {
            self.code_review_summaries = input;
            self
        }
        /// <p>
        /// Pagination token.
        /// </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListCodeReviewsOutput`](crate::output::ListCodeReviewsOutput)
        pub fn build(self) -> crate::output::ListCodeReviewsOutput {
            crate::output::ListCodeReviewsOutput {
                code_review_summaries: self.code_review_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListCodeReviewsOutput {
    /// Creates a new builder-style object to manufacture [`ListCodeReviewsOutput`](crate::output::ListCodeReviewsOutput)
    pub fn builder() -> crate::output::list_code_reviews_output::Builder {
        crate::output::list_code_reviews_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DisassociateRepositoryOutput {
    /// <p>Information about the disassociated repository.</p>
    pub repository_association: std::option::Option<crate::model::RepositoryAssociation>,
    /// <p>
    /// An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:
    /// </p>
    /// <ul>
    /// <li>
    /// <p>A <i>tag key</i> (for example, <code>CostCenter</code>,
    /// <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
    /// keys are case sensitive.</p>
    /// </li>
    /// <li>
    /// <p>An optional field known as a <i>tag value</i> (for example,
    /// <code>111122223333</code>, <code>Production</code>, or a team name).
    /// Omitting the tag value is the same as using an empty string. Like tag keys, tag
    /// values are case sensitive.</p>
    /// </li>
    /// </ul>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for DisassociateRepositoryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DisassociateRepositoryOutput");
        formatter.field("repository_association", &self.repository_association);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`DisassociateRepositoryOutput`](crate::output::DisassociateRepositoryOutput)
pub mod disassociate_repository_output {
    /// A builder for [`DisassociateRepositoryOutput`](crate::output::DisassociateRepositoryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) repository_association: std::option::Option<crate::model::RepositoryAssociation>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>Information about the disassociated repository.</p>
        pub fn repository_association(
            mut self,
            input: crate::model::RepositoryAssociation,
        ) -> Self {
            self.repository_association = Some(input);
            self
        }
        pub fn set_repository_association(
            mut self,
            input: std::option::Option<crate::model::RepositoryAssociation>,
        ) -> Self {
            self.repository_association = input;
            self
        }
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`DisassociateRepositoryOutput`](crate::output::DisassociateRepositoryOutput)
        pub fn build(self) -> crate::output::DisassociateRepositoryOutput {
            crate::output::DisassociateRepositoryOutput {
                repository_association: self.repository_association,
                tags: self.tags,
            }
        }
    }
}
impl DisassociateRepositoryOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateRepositoryOutput`](crate::output::DisassociateRepositoryOutput)
    pub fn builder() -> crate::output::disassociate_repository_output::Builder {
        crate::output::disassociate_repository_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeRepositoryAssociationOutput {
    /// <p>Information about the repository association.</p>
    pub repository_association: std::option::Option<crate::model::RepositoryAssociation>,
    /// <p>
    /// An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:
    /// </p>
    /// <ul>
    /// <li>
    /// <p>A <i>tag key</i> (for example, <code>CostCenter</code>,
    /// <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
    /// keys are case sensitive.</p>
    /// </li>
    /// <li>
    /// <p>An optional field known as a <i>tag value</i> (for example,
    /// <code>111122223333</code>, <code>Production</code>, or a team name).
    /// Omitting the tag value is the same as using an empty string. Like tag keys, tag
    /// values are case sensitive.</p>
    /// </li>
    /// </ul>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for DescribeRepositoryAssociationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeRepositoryAssociationOutput");
        formatter.field("repository_association", &self.repository_association);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`DescribeRepositoryAssociationOutput`](crate::output::DescribeRepositoryAssociationOutput)
pub mod describe_repository_association_output {
    /// A builder for [`DescribeRepositoryAssociationOutput`](crate::output::DescribeRepositoryAssociationOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) repository_association: std::option::Option<crate::model::RepositoryAssociation>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>Information about the repository association.</p>
        pub fn repository_association(
            mut self,
            input: crate::model::RepositoryAssociation,
        ) -> Self {
            self.repository_association = Some(input);
            self
        }
        pub fn set_repository_association(
            mut self,
            input: std::option::Option<crate::model::RepositoryAssociation>,
        ) -> Self {
            self.repository_association = input;
            self
        }
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeRepositoryAssociationOutput`](crate::output::DescribeRepositoryAssociationOutput)
        pub fn build(self) -> crate::output::DescribeRepositoryAssociationOutput {
            crate::output::DescribeRepositoryAssociationOutput {
                repository_association: self.repository_association,
                tags: self.tags,
            }
        }
    }
}
impl DescribeRepositoryAssociationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRepositoryAssociationOutput`](crate::output::DescribeRepositoryAssociationOutput)
    pub fn builder() -> crate::output::describe_repository_association_output::Builder {
        crate::output::describe_repository_association_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeRecommendationFeedbackOutput {
    /// <p>
    /// The recommendation feedback given by the user.
    /// </p>
    pub recommendation_feedback: std::option::Option<crate::model::RecommendationFeedback>,
}
impl std::fmt::Debug for DescribeRecommendationFeedbackOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeRecommendationFeedbackOutput");
        formatter.field("recommendation_feedback", &self.recommendation_feedback);
        formatter.finish()
    }
}
/// See [`DescribeRecommendationFeedbackOutput`](crate::output::DescribeRecommendationFeedbackOutput)
pub mod describe_recommendation_feedback_output {
    /// A builder for [`DescribeRecommendationFeedbackOutput`](crate::output::DescribeRecommendationFeedbackOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) recommendation_feedback:
            std::option::Option<crate::model::RecommendationFeedback>,
    }
    impl Builder {
        /// <p>
        /// The recommendation feedback given by the user.
        /// </p>
        pub fn recommendation_feedback(
            mut self,
            input: crate::model::RecommendationFeedback,
        ) -> Self {
            self.recommendation_feedback = Some(input);
            self
        }
        pub fn set_recommendation_feedback(
            mut self,
            input: std::option::Option<crate::model::RecommendationFeedback>,
        ) -> Self {
            self.recommendation_feedback = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeRecommendationFeedbackOutput`](crate::output::DescribeRecommendationFeedbackOutput)
        pub fn build(self) -> crate::output::DescribeRecommendationFeedbackOutput {
            crate::output::DescribeRecommendationFeedbackOutput {
                recommendation_feedback: self.recommendation_feedback,
            }
        }
    }
}
impl DescribeRecommendationFeedbackOutput {
    /// Creates a new builder-style object to manufacture [`DescribeRecommendationFeedbackOutput`](crate::output::DescribeRecommendationFeedbackOutput)
    pub fn builder() -> crate::output::describe_recommendation_feedback_output::Builder {
        crate::output::describe_recommendation_feedback_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeCodeReviewOutput {
    /// <p>
    /// Information about the code review.
    /// </p>
    pub code_review: std::option::Option<crate::model::CodeReview>,
}
impl std::fmt::Debug for DescribeCodeReviewOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeCodeReviewOutput");
        formatter.field("code_review", &self.code_review);
        formatter.finish()
    }
}
/// See [`DescribeCodeReviewOutput`](crate::output::DescribeCodeReviewOutput)
pub mod describe_code_review_output {
    /// A builder for [`DescribeCodeReviewOutput`](crate::output::DescribeCodeReviewOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) code_review: std::option::Option<crate::model::CodeReview>,
    }
    impl Builder {
        /// <p>
        /// Information about the code review.
        /// </p>
        pub fn code_review(mut self, input: crate::model::CodeReview) -> Self {
            self.code_review = Some(input);
            self
        }
        pub fn set_code_review(
            mut self,
            input: std::option::Option<crate::model::CodeReview>,
        ) -> Self {
            self.code_review = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeCodeReviewOutput`](crate::output::DescribeCodeReviewOutput)
        pub fn build(self) -> crate::output::DescribeCodeReviewOutput {
            crate::output::DescribeCodeReviewOutput {
                code_review: self.code_review,
            }
        }
    }
}
impl DescribeCodeReviewOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCodeReviewOutput`](crate::output::DescribeCodeReviewOutput)
    pub fn builder() -> crate::output::describe_code_review_output::Builder {
        crate::output::describe_code_review_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateCodeReviewOutput {
    /// <p>
    /// Information about a code review. A code review belongs to the associated repository that contains the reviewed code.
    /// </p>
    pub code_review: std::option::Option<crate::model::CodeReview>,
}
impl std::fmt::Debug for CreateCodeReviewOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateCodeReviewOutput");
        formatter.field("code_review", &self.code_review);
        formatter.finish()
    }
}
/// See [`CreateCodeReviewOutput`](crate::output::CreateCodeReviewOutput)
pub mod create_code_review_output {
    /// A builder for [`CreateCodeReviewOutput`](crate::output::CreateCodeReviewOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) code_review: std::option::Option<crate::model::CodeReview>,
    }
    impl Builder {
        /// <p>
        /// Information about a code review. A code review belongs to the associated repository that contains the reviewed code.
        /// </p>
        pub fn code_review(mut self, input: crate::model::CodeReview) -> Self {
            self.code_review = Some(input);
            self
        }
        pub fn set_code_review(
            mut self,
            input: std::option::Option<crate::model::CodeReview>,
        ) -> Self {
            self.code_review = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateCodeReviewOutput`](crate::output::CreateCodeReviewOutput)
        pub fn build(self) -> crate::output::CreateCodeReviewOutput {
            crate::output::CreateCodeReviewOutput {
                code_review: self.code_review,
            }
        }
    }
}
impl CreateCodeReviewOutput {
    /// Creates a new builder-style object to manufacture [`CreateCodeReviewOutput`](crate::output::CreateCodeReviewOutput)
    pub fn builder() -> crate::output::create_code_review_output::Builder {
        crate::output::create_code_review_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AssociateRepositoryOutput {
    /// <p>Information about the repository association.</p>
    pub repository_association: std::option::Option<crate::model::RepositoryAssociation>,
    /// <p>
    /// An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:
    /// </p>
    /// <ul>
    /// <li>
    /// <p>A <i>tag key</i> (for example, <code>CostCenter</code>,
    /// <code>Environment</code>, <code>Project</code>, or <code>Secret</code>). Tag
    /// keys are case sensitive.</p>
    /// </li>
    /// <li>
    /// <p>An optional field known as a <i>tag value</i> (for example,
    /// <code>111122223333</code>, <code>Production</code>, or a team name).
    /// Omitting the tag value is the same as using an empty string. Like tag keys, tag
    /// values are case sensitive.</p>
    /// </li>
    /// </ul>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for AssociateRepositoryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AssociateRepositoryOutput");
        formatter.field("repository_association", &self.repository_association);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`AssociateRepositoryOutput`](crate::output::AssociateRepositoryOutput)
pub mod associate_repository_output {
    /// A builder for [`AssociateRepositoryOutput`](crate::output::AssociateRepositoryOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) repository_association: std::option::Option<crate::model::RepositoryAssociation>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>Information about the repository association.</p>
        pub fn repository_association(
            mut self,
            input: crate::model::RepositoryAssociation,
        ) -> Self {
            self.repository_association = Some(input);
            self
        }
        pub fn set_repository_association(
            mut self,
            input: std::option::Option<crate::model::RepositoryAssociation>,
        ) -> Self {
            self.repository_association = input;
            self
        }
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`AssociateRepositoryOutput`](crate::output::AssociateRepositoryOutput)
        pub fn build(self) -> crate::output::AssociateRepositoryOutput {
            crate::output::AssociateRepositoryOutput {
                repository_association: self.repository_association,
                tags: self.tags,
            }
        }
    }
}
impl AssociateRepositoryOutput {
    /// Creates a new builder-style object to manufacture [`AssociateRepositoryOutput`](crate::output::AssociateRepositoryOutput)
    pub fn builder() -> crate::output::associate_repository_output::Builder {
        crate::output::associate_repository_output::Builder::default()
    }
}
