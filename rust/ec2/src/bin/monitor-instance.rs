#![allow(clippy::result_large_err)]

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_ec2::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    // The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    // The iD of the instance to monitor.
    #[structopt(short, long)]
    instance_id: String,

    #[structopt(short, long)]
    verbose: bool,
}

async fn enable_monitoring(client: &Client, id: &str) -> Result<(), Error> {
    client.monitor_instances().instance_ids(id).send().await?;

    println!("Enabled monitoring");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        region,
        instance_id,
        verbose,
    } = Opt::parse();

    let region_provider: RegionProviderChain =
        RegionProviderChain::first_try(region.map(Region::new))
            .or_default_provider()
            .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("EC2 client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Instance ID:        {}", instance_id);
        println!();
    }

    let config: SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let client: Client = Client::new(&config);

    enable_monitoring(&client, &instance_id).await
}
