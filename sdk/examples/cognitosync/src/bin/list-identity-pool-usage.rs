/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitosync::{Client, Error, Region, PKG_VERSION};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

/// Lists the identity pools registered with Amazon Cognito in the Region.
/// # Arguments
///
/// * `[-r REGION]` - The region containing the buckets.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-g]` - Whether to display buckets in all regions.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt { region, verbose } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    println!();

    if verbose {
        println!("Cognito client version: {}", PKG_VERSION);
        println!(
            "Region:                 {}",
            shared_config.region().unwrap()
        );
        println!();
    }

    let response = client
        .list_identity_pool_usage()
        .max_results(10)
        .send()
        .await?;

    if let Some(pools) = response.identity_pool_usages {
        println!("Identity pools:");

        for pool in pools {
            println!(
                "  Identity pool ID:    {}",
                pool.identity_pool_id.unwrap_or_default()
            );
            println!(
                "  Data storage:        {}",
                pool.data_storage.unwrap_or_default()
            );
            println!(
                "  Sync sessions count: {}",
                pool.sync_sessions_count.unwrap_or_default()
            );
            println!(
                "  Last modified:       {}",
                pool.last_modified_date.unwrap().to_chrono()
            );
            println!();
        }
    }

    println!("Next token: {:?}", response.next_token);

    Ok(())
}
