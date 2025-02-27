// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a budget and, if included, notifications and subscribers. </p>
/// <important>
/// <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_CreateBudget.html#API_CreateBudget_Examples">Examples</a> section. </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateBudget {
    _private: (),
}
impl CreateBudget {
    /// Creates a new builder-style object to manufacture [`CreateBudgetInput`](crate::input::CreateBudgetInput)
    pub fn builder() -> crate::input::create_budget_input::Builder {
        crate::input::create_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateBudget {
    type Output =
        std::result::Result<crate::output::CreateBudgetOutput, crate::error::CreateBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_budget_error(response)
        } else {
            crate::operation_deser::parse_create_budget_response(response)
        }
    }
}

/// <p>
/// Creates a budget action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateBudgetAction {
    _private: (),
}
impl CreateBudgetAction {
    /// Creates a new builder-style object to manufacture [`CreateBudgetActionInput`](crate::input::CreateBudgetActionInput)
    pub fn builder() -> crate::input::create_budget_action_input::Builder {
        crate::input::create_budget_action_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateBudgetAction {
    type Output = std::result::Result<
        crate::output::CreateBudgetActionOutput,
        crate::error::CreateBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_budget_action_error(response)
        } else {
            crate::operation_deser::parse_create_budget_action_response(response)
        }
    }
}

/// <p>Creates a notification. You must create the budget before you create the associated notification.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateNotification {
    _private: (),
}
impl CreateNotification {
    /// Creates a new builder-style object to manufacture [`CreateNotificationInput`](crate::input::CreateNotificationInput)
    pub fn builder() -> crate::input::create_notification_input::Builder {
        crate::input::create_notification_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateNotification {
    type Output = std::result::Result<
        crate::output::CreateNotificationOutput,
        crate::error::CreateNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_notification_error(response)
        } else {
            crate::operation_deser::parse_create_notification_response(response)
        }
    }
}

/// <p>Creates a subscriber. You must create the associated budget and notification before you create the subscriber.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSubscriber {
    _private: (),
}
impl CreateSubscriber {
    /// Creates a new builder-style object to manufacture [`CreateSubscriberInput`](crate::input::CreateSubscriberInput)
    pub fn builder() -> crate::input::create_subscriber_input::Builder {
        crate::input::create_subscriber_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateSubscriber {
    type Output = std::result::Result<
        crate::output::CreateSubscriberOutput,
        crate::error::CreateSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_subscriber_error(response)
        } else {
            crate::operation_deser::parse_create_subscriber_response(response)
        }
    }
}

/// <p>Deletes a budget. You can delete your budget at any time.</p>
/// <important>
/// <p>Deleting a budget also deletes the notifications and subscribers that are associated with that budget.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBudget {
    _private: (),
}
impl DeleteBudget {
    /// Creates a new builder-style object to manufacture [`DeleteBudgetInput`](crate::input::DeleteBudgetInput)
    pub fn builder() -> crate::input::delete_budget_input::Builder {
        crate::input::delete_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteBudget {
    type Output =
        std::result::Result<crate::output::DeleteBudgetOutput, crate::error::DeleteBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_budget_error(response)
        } else {
            crate::operation_deser::parse_delete_budget_response(response)
        }
    }
}

/// <p>
/// Deletes a budget action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBudgetAction {
    _private: (),
}
impl DeleteBudgetAction {
    /// Creates a new builder-style object to manufacture [`DeleteBudgetActionInput`](crate::input::DeleteBudgetActionInput)
    pub fn builder() -> crate::input::delete_budget_action_input::Builder {
        crate::input::delete_budget_action_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteBudgetAction {
    type Output = std::result::Result<
        crate::output::DeleteBudgetActionOutput,
        crate::error::DeleteBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_budget_action_error(response)
        } else {
            crate::operation_deser::parse_delete_budget_action_response(response)
        }
    }
}

/// <p>Deletes a notification.</p>
/// <important>
/// <p>Deleting a notification also deletes the subscribers that are associated with the notification.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteNotification {
    _private: (),
}
impl DeleteNotification {
    /// Creates a new builder-style object to manufacture [`DeleteNotificationInput`](crate::input::DeleteNotificationInput)
    pub fn builder() -> crate::input::delete_notification_input::Builder {
        crate::input::delete_notification_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteNotification {
    type Output = std::result::Result<
        crate::output::DeleteNotificationOutput,
        crate::error::DeleteNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_notification_error(response)
        } else {
            crate::operation_deser::parse_delete_notification_response(response)
        }
    }
}

/// <p>Deletes a subscriber.</p>
/// <important>
/// <p>Deleting the last subscriber to a notification also deletes the notification.</p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSubscriber {
    _private: (),
}
impl DeleteSubscriber {
    /// Creates a new builder-style object to manufacture [`DeleteSubscriberInput`](crate::input::DeleteSubscriberInput)
    pub fn builder() -> crate::input::delete_subscriber_input::Builder {
        crate::input::delete_subscriber_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSubscriber {
    type Output = std::result::Result<
        crate::output::DeleteSubscriberOutput,
        crate::error::DeleteSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_subscriber_error(response)
        } else {
            crate::operation_deser::parse_delete_subscriber_response(response)
        }
    }
}

/// <p>Describes a budget.</p>
/// <important>
/// <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudget.html#API_DescribeBudget_Examples">Examples</a> section. </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudget {
    _private: (),
}
impl DescribeBudget {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetInput`](crate::input::DescribeBudgetInput)
    pub fn builder() -> crate::input::describe_budget_input::Builder {
        crate::input::describe_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudget {
    type Output =
        std::result::Result<crate::output::DescribeBudgetOutput, crate::error::DescribeBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_response(response)
        }
    }
}

/// <p>
/// Describes a budget action detail.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetAction {
    _private: (),
}
impl DescribeBudgetAction {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionInput`](crate::input::DescribeBudgetActionInput)
    pub fn builder() -> crate::input::describe_budget_action_input::Builder {
        crate::input::describe_budget_action_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgetAction {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionOutput,
        crate::error::DescribeBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_action_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_action_response(response)
        }
    }
}

/// <p>
/// Describes a budget action history detail.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionHistories {
    _private: (),
}
impl DescribeBudgetActionHistories {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionHistoriesInput`](crate::input::DescribeBudgetActionHistoriesInput)
    pub fn builder() -> crate::input::describe_budget_action_histories_input::Builder {
        crate::input::describe_budget_action_histories_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgetActionHistories {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionHistoriesOutput,
        crate::error::DescribeBudgetActionHistoriesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_action_histories_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_action_histories_response(response)
        }
    }
}

/// <p>
/// Describes all of the budget actions for an account.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionsForAccount {
    _private: (),
}
impl DescribeBudgetActionsForAccount {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionsForAccountInput`](crate::input::DescribeBudgetActionsForAccountInput)
    pub fn builder() -> crate::input::describe_budget_actions_for_account_input::Builder {
        crate::input::describe_budget_actions_for_account_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgetActionsForAccount {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionsForAccountOutput,
        crate::error::DescribeBudgetActionsForAccountError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_actions_for_account_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_actions_for_account_response(response)
        }
    }
}

/// <p>
/// Describes all of the budget actions for a budget.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionsForBudget {
    _private: (),
}
impl DescribeBudgetActionsForBudget {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionsForBudgetInput`](crate::input::DescribeBudgetActionsForBudgetInput)
    pub fn builder() -> crate::input::describe_budget_actions_for_budget_input::Builder {
        crate::input::describe_budget_actions_for_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgetActionsForBudget {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionsForBudgetOutput,
        crate::error::DescribeBudgetActionsForBudgetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_actions_for_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_actions_for_budget_response(response)
        }
    }
}

/// <p>Describes the history for <code>DAILY</code>, <code>MONTHLY</code>, and <code>QUARTERLY</code> budgets. Budget history isn't available for <code>ANNUAL</code> budgets.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetPerformanceHistory {
    _private: (),
}
impl DescribeBudgetPerformanceHistory {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetPerformanceHistoryInput`](crate::input::DescribeBudgetPerformanceHistoryInput)
    pub fn builder() -> crate::input::describe_budget_performance_history_input::Builder {
        crate::input::describe_budget_performance_history_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgetPerformanceHistory {
    type Output = std::result::Result<
        crate::output::DescribeBudgetPerformanceHistoryOutput,
        crate::error::DescribeBudgetPerformanceHistoryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_performance_history_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_performance_history_response(response)
        }
    }
}

/// <p>Lists the budgets that are associated with an account.</p>
/// <important>
/// <p>The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_DescribeBudgets.html#API_DescribeBudgets_Examples">Examples</a> section. </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgets {
    _private: (),
}
impl DescribeBudgets {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetsInput`](crate::input::DescribeBudgetsInput)
    pub fn builder() -> crate::input::describe_budgets_input::Builder {
        crate::input::describe_budgets_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBudgets {
    type Output = std::result::Result<
        crate::output::DescribeBudgetsOutput,
        crate::error::DescribeBudgetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budgets_error(response)
        } else {
            crate::operation_deser::parse_describe_budgets_response(response)
        }
    }
}

/// <p>Lists the notifications that are associated with a budget.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeNotificationsForBudget {
    _private: (),
}
impl DescribeNotificationsForBudget {
    /// Creates a new builder-style object to manufacture [`DescribeNotificationsForBudgetInput`](crate::input::DescribeNotificationsForBudgetInput)
    pub fn builder() -> crate::input::describe_notifications_for_budget_input::Builder {
        crate::input::describe_notifications_for_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeNotificationsForBudget {
    type Output = std::result::Result<
        crate::output::DescribeNotificationsForBudgetOutput,
        crate::error::DescribeNotificationsForBudgetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_notifications_for_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_notifications_for_budget_response(response)
        }
    }
}

/// <p>Lists the subscribers that are associated with a notification.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSubscribersForNotification {
    _private: (),
}
impl DescribeSubscribersForNotification {
    /// Creates a new builder-style object to manufacture [`DescribeSubscribersForNotificationInput`](crate::input::DescribeSubscribersForNotificationInput)
    pub fn builder() -> crate::input::describe_subscribers_for_notification_input::Builder {
        crate::input::describe_subscribers_for_notification_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSubscribersForNotification {
    type Output = std::result::Result<
        crate::output::DescribeSubscribersForNotificationOutput,
        crate::error::DescribeSubscribersForNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_subscribers_for_notification_error(response)
        } else {
            crate::operation_deser::parse_describe_subscribers_for_notification_response(response)
        }
    }
}

/// <p>
/// Executes a budget action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExecuteBudgetAction {
    _private: (),
}
impl ExecuteBudgetAction {
    /// Creates a new builder-style object to manufacture [`ExecuteBudgetActionInput`](crate::input::ExecuteBudgetActionInput)
    pub fn builder() -> crate::input::execute_budget_action_input::Builder {
        crate::input::execute_budget_action_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ExecuteBudgetAction {
    type Output = std::result::Result<
        crate::output::ExecuteBudgetActionOutput,
        crate::error::ExecuteBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_execute_budget_action_error(response)
        } else {
            crate::operation_deser::parse_execute_budget_action_response(response)
        }
    }
}

/// <p>Updates a budget. You can change every part of a budget except for the <code>budgetName</code> and the <code>calculatedSpend</code>. When you modify a budget, the <code>calculatedSpend</code> drops to zero until AWS has new usage data to use for forecasting.</p>
/// <important>
/// <p>Only one of <code>BudgetLimit</code> or <code>PlannedBudgetLimits</code> can be present in the syntax at one time. Use the syntax that matches your case. The Request Syntax section shows the <code>BudgetLimit</code> syntax. For <code>PlannedBudgetLimits</code>, see the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_UpdateBudget.html#API_UpdateBudget_Examples">Examples</a> section. </p>
/// </important>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBudget {
    _private: (),
}
impl UpdateBudget {
    /// Creates a new builder-style object to manufacture [`UpdateBudgetInput`](crate::input::UpdateBudgetInput)
    pub fn builder() -> crate::input::update_budget_input::Builder {
        crate::input::update_budget_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateBudget {
    type Output =
        std::result::Result<crate::output::UpdateBudgetOutput, crate::error::UpdateBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_budget_error(response)
        } else {
            crate::operation_deser::parse_update_budget_response(response)
        }
    }
}

/// <p>
/// Updates a budget action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBudgetAction {
    _private: (),
}
impl UpdateBudgetAction {
    /// Creates a new builder-style object to manufacture [`UpdateBudgetActionInput`](crate::input::UpdateBudgetActionInput)
    pub fn builder() -> crate::input::update_budget_action_input::Builder {
        crate::input::update_budget_action_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateBudgetAction {
    type Output = std::result::Result<
        crate::output::UpdateBudgetActionOutput,
        crate::error::UpdateBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_budget_action_error(response)
        } else {
            crate::operation_deser::parse_update_budget_action_response(response)
        }
    }
}

/// <p>Updates a notification.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNotification {
    _private: (),
}
impl UpdateNotification {
    /// Creates a new builder-style object to manufacture [`UpdateNotificationInput`](crate::input::UpdateNotificationInput)
    pub fn builder() -> crate::input::update_notification_input::Builder {
        crate::input::update_notification_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateNotification {
    type Output = std::result::Result<
        crate::output::UpdateNotificationOutput,
        crate::error::UpdateNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_notification_error(response)
        } else {
            crate::operation_deser::parse_update_notification_response(response)
        }
    }
}

/// <p>Updates a subscriber.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSubscriber {
    _private: (),
}
impl UpdateSubscriber {
    /// Creates a new builder-style object to manufacture [`UpdateSubscriberInput`](crate::input::UpdateSubscriberInput)
    pub fn builder() -> crate::input::update_subscriber_input::Builder {
        crate::input::update_subscriber_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateSubscriber {
    type Output = std::result::Result<
        crate::output::UpdateSubscriberOutput,
        crate::error::UpdateSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_subscriber_error(response)
        } else {
            crate::operation_deser::parse_update_subscriber_response(response)
        }
    }
}
