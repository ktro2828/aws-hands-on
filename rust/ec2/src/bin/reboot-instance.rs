#![allow(clippy::result_large_err)]

use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_ec2::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    // The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    // The ID of the instance to reboot.
    #[structopt(short, long)]
    instance_id: String,

    // Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

async fn reboot_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.reboot_instances().instance_ids(id).send().await?;
    println!("Rebooted instance.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {region, instance_id, verbose}  = Opt::parse();

    let region_provider: RegionProviderChain = RegionProviderChain::first_try(region.map(Region::new)).or_default_provider().or_else(Region::new("us-west-2"));
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

    reboot_instance(&client, &instance_id).await
}