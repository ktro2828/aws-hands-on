#![allow(clippy::result_large_err)]

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_ec2::{config::Region, meta::PKG_VERSION, Client, Error, operation::describe_instances::DescribeInstancesOutput};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// To get info about one instance
    #[structopt(short, long)]
    instance_id: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<(), Error> {
    let resp: DescribeInstancesOutput = client.describe_instances().set_instance_ids(ids).send().await?;

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            println!("Instance ID: {}", instance.instance_id().unwrap());
            println!(
                "State:     {:?}",
                instance.state().unwrap().name().unwrap()
            );
            println!();
        }
    }

    Ok(())
}

// snippet-end:[ec2.rust.describe-instances]

/// Lists the state of one or all of your Amazon EC2 instances.
/// # Arguments
///
/// * `[-i INSTANCE-ID]` - The ID of an instance.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Args {region, instance_id, verbose} = Args::parse();

    let region_provider: RegionProviderChain = RegionProviderChain::first_try(region.map(Region::new)).or_default_provider().or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("EC2 client version: {}", PKG_VERSION);
        println!(
            "Region:        {}",
            region_provider.region().await.unwrap().as_ref()
        );

        if instance_id.is_some() {
            println!("Instance ID:          {:?}", instance_id);
        }

        println!();
    }

    let shared_config: SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let client: Client = Client::new(&shared_config);

    let ids: Option<Vec<String>> = instance_id.map(|id:String| vec![id]);

    show_state(&client, ids).await
}