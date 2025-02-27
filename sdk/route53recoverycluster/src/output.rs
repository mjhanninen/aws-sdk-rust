// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateRoutingControlStatesOutput {}
impl std::fmt::Debug for UpdateRoutingControlStatesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateRoutingControlStatesOutput");
        formatter.finish()
    }
}
/// See [`UpdateRoutingControlStatesOutput`](crate::output::UpdateRoutingControlStatesOutput)
pub mod update_routing_control_states_output {
    /// A builder for [`UpdateRoutingControlStatesOutput`](crate::output::UpdateRoutingControlStatesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateRoutingControlStatesOutput`](crate::output::UpdateRoutingControlStatesOutput)
        pub fn build(self) -> crate::output::UpdateRoutingControlStatesOutput {
            crate::output::UpdateRoutingControlStatesOutput {}
        }
    }
}
impl UpdateRoutingControlStatesOutput {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStatesOutput`](crate::output::UpdateRoutingControlStatesOutput)
    pub fn builder() -> crate::output::update_routing_control_states_output::Builder {
        crate::output::update_routing_control_states_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateRoutingControlStateOutput {}
impl std::fmt::Debug for UpdateRoutingControlStateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateRoutingControlStateOutput");
        formatter.finish()
    }
}
/// See [`UpdateRoutingControlStateOutput`](crate::output::UpdateRoutingControlStateOutput)
pub mod update_routing_control_state_output {
    /// A builder for [`UpdateRoutingControlStateOutput`](crate::output::UpdateRoutingControlStateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateRoutingControlStateOutput`](crate::output::UpdateRoutingControlStateOutput)
        pub fn build(self) -> crate::output::UpdateRoutingControlStateOutput {
            crate::output::UpdateRoutingControlStateOutput {}
        }
    }
}
impl UpdateRoutingControlStateOutput {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStateOutput`](crate::output::UpdateRoutingControlStateOutput)
    pub fn builder() -> crate::output::update_routing_control_state_output::Builder {
        crate::output::update_routing_control_state_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRoutingControlStateOutput {
    /// <p>The Amazon Resource Number (ARN) of the response.</p>
    pub routing_control_arn: std::option::Option<std::string::String>,
    /// <p>The state of the routing control.</p>
    pub routing_control_state: std::option::Option<crate::model::RoutingControlState>,
}
impl std::fmt::Debug for GetRoutingControlStateOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoutingControlStateOutput");
        formatter.field("routing_control_arn", &self.routing_control_arn);
        formatter.field("routing_control_state", &self.routing_control_state);
        formatter.finish()
    }
}
/// See [`GetRoutingControlStateOutput`](crate::output::GetRoutingControlStateOutput)
pub mod get_routing_control_state_output {
    /// A builder for [`GetRoutingControlStateOutput`](crate::output::GetRoutingControlStateOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) routing_control_arn: std::option::Option<std::string::String>,
        pub(crate) routing_control_state: std::option::Option<crate::model::RoutingControlState>,
    }
    impl Builder {
        /// <p>The Amazon Resource Number (ARN) of the response.</p>
        pub fn routing_control_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.routing_control_arn = Some(input.into());
            self
        }
        pub fn set_routing_control_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.routing_control_arn = input;
            self
        }
        /// <p>The state of the routing control.</p>
        pub fn routing_control_state(mut self, input: crate::model::RoutingControlState) -> Self {
            self.routing_control_state = Some(input);
            self
        }
        pub fn set_routing_control_state(
            mut self,
            input: std::option::Option<crate::model::RoutingControlState>,
        ) -> Self {
            self.routing_control_state = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRoutingControlStateOutput`](crate::output::GetRoutingControlStateOutput)
        pub fn build(self) -> crate::output::GetRoutingControlStateOutput {
            crate::output::GetRoutingControlStateOutput {
                routing_control_arn: self.routing_control_arn,
                routing_control_state: self.routing_control_state,
            }
        }
    }
}
impl GetRoutingControlStateOutput {
    /// Creates a new builder-style object to manufacture [`GetRoutingControlStateOutput`](crate::output::GetRoutingControlStateOutput)
    pub fn builder() -> crate::output::get_routing_control_state_output::Builder {
        crate::output::get_routing_control_state_output::Builder::default()
    }
}
