/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;

use sagemaker::{Client, Region};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region. Overrides environment variable AWS_DEFAULT_REGION.
    #[structopt(short, long)]
    default_region: Option<String>,

    /// Whether to display additional runtime information
    #[structopt(short, long)]
    verbose: bool,
}

/// Lists the your SageMaker jobs in an AWS Region.
/// /// # Arguments
///
/// * `[-d DEFAULT-REGION]` - The region in which the client is created.
///    If not supplied, uses the value of the **AWS_DEFAULT_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), sagemaker::Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        default_region,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(default_region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    if verbose {
        println!("SageMaker client version: {}", sagemaker::PKG_VERSION);
        println!(
            "Region:                   {:?}",
            shared_config.region().unwrap()
        );
        println!();
    }

    let job_details = client.list_training_jobs().send().await?;

    println!("Job Name\tCreation DateTime\tDuration\tStatus");
    for j in job_details.training_job_summaries.unwrap_or_default() {
        let name = j.training_job_name.as_deref().unwrap_or_default();
        let creation_time = j.creation_time.unwrap().to_chrono();
        let training_end_time = j.training_end_time.unwrap().to_chrono();

        let status = j.training_job_status.unwrap();
        let duration = training_end_time - creation_time;

        println!(
            "{}\t{}\t{}\t{:#?}",
            name,
            creation_time.format("%Y-%m-%d@%H:%M:%S"),
            duration.num_seconds(),
            status
        );
    }

    Ok(())
}
