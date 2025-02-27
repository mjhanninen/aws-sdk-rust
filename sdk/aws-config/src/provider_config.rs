/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

//! Configuration Options for Credential Providers

use crate::connector::default_connector;
use crate::default_provider::region::DefaultRegionChain;
use aws_types::os_shim_internal::{Env, Fs};
use aws_types::region::Region;
use smithy_async::rt::sleep::{default_async_sleep, AsyncSleep};
use smithy_client::erase::DynConnector;
use std::sync::Arc;

/// Configuration options for Credential Providers
///
/// Most credential providers builders offer a `configure` method which applies general provider configuration
/// options.
///
/// To use a region from the default region provider chain use [`ProviderConfig::with_default_region`].
/// Otherwise, use [`ProviderConfig::without_region`]. Note that some credentials providers require a region
/// to be explicitly set.
#[derive(Clone)]
pub struct ProviderConfig {
    env: Env,
    fs: Fs,
    connector: Option<DynConnector>,
    sleep: Option<Arc<dyn AsyncSleep>>,
    region: Option<Region>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            env: Env::default(),
            fs: Fs::default(),
            connector: default_connector(),
            sleep: default_async_sleep(),
            region: None,
        }
    }
}

impl ProviderConfig {
    /// Create a default provider config with the region unset.
    ///
    /// Using this option means that you may need to set a region manually.
    ///
    /// This constructor will use a default value for the HTTPS connector and Sleep implementation
    /// when they are enabled as crate features which is usually the correct option. To construct
    /// a `ProviderConfig` without these fields set, use [`ProviderConfig::empty`].
    ///
    ///
    /// # Example
    /// ```rust
    /// use aws_config::provider_config::ProviderConfig;
    /// use aws_sdk_sts::Region;
    /// use aws_config::web_identity_token::WebIdentityTokenCredentialsProvider;
    /// let conf = ProviderConfig::without_region().with_region(Some(Region::new("us-east-1")));
    ///
    /// # if cfg!(any(feature = "rustls", feature = "native-tls")) {
    /// let credential_provider = WebIdentityTokenCredentialsProvider::builder().configure(&conf).build();
    /// # }
    /// ```
    pub fn without_region() -> Self {
        Self::default()
    }

    /// Constructs a ProviderConfig with no fields set
    pub fn empty() -> Self {
        ProviderConfig {
            env: Env::default(),
            fs: Fs::default(),
            connector: None,
            sleep: None,
            region: None,
        }
    }

    /// Create a default provider config with the region region automatically loaded from the default chain.
    ///
    /// # Example
    /// ```rust
    /// # async fn test() {
    /// use aws_config::provider_config::ProviderConfig;
    /// use aws_sdk_sts::Region;
    /// use aws_config::web_identity_token::WebIdentityTokenCredentialsProvider;
    /// let conf = ProviderConfig::with_default_region().await;
    /// let credential_provider = WebIdentityTokenCredentialsProvider::builder().configure(&conf).build();
    /// }
    /// ```
    #[cfg(feature = "default-provider")]
    pub async fn with_default_region() -> Self {
        Self::without_region().load_default_region().await
    }

    // When all crate features are disabled, these accessors are unused

    #[allow(dead_code)]
    pub(crate) fn env(&self) -> Env {
        self.env.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn fs(&self) -> Fs {
        self.fs.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn connector(&self) -> Option<&DynConnector> {
        self.connector.as_ref()
    }

    #[allow(dead_code)]
    pub(crate) fn sleep(&self) -> Option<Arc<dyn AsyncSleep>> {
        self.sleep.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn region(&self) -> Option<Region> {
        self.region.clone()
    }

    /// Override the region for the configuration
    pub fn with_region(mut self, region: Option<Region>) -> Self {
        self.region = region;
        self
    }

    #[cfg(feature = "default-provider")]
    /// Use the [default region chain](crate::default_provider::region) to set the
    /// region for this configuration
    ///
    /// Note: the `env` and `fs` already set on this provider will be used when loading the default region.
    pub async fn load_default_region(self) -> Self {
        let provider_chain = DefaultRegionChain::builder().configure(&self).build();
        self.with_region(provider_chain.region().await)
    }

    #[doc(hidden)]
    pub fn with_fs(self, fs: Fs) -> Self {
        ProviderConfig { fs, ..self }
    }

    #[doc(hidden)]
    pub fn with_env(self, env: Env) -> Self {
        ProviderConfig { env, ..self }
    }

    /// Override the HTTPS connector for this configuration
    ///
    /// ## Note: Stability
    /// This method is expected to change to support HTTP configuration
    pub fn with_connector(self, connector: DynConnector) -> Self {
        ProviderConfig {
            connector: Some(connector),
            ..self
        }
    }

    /// Override the sleep implementation for this configuration
    pub fn with_sleep(self, sleep: impl AsyncSleep + 'static) -> Self {
        ProviderConfig {
            sleep: Some(Arc::new(sleep)),
            ..self
        }
    }
}
