#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecs::{config::Region, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    // The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    // The name of the cluster.
    #[structopt(short, long)]
    name: String,

    // Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

async fn make_cluster(client: &Client, name: &str) -> Result<(), Error> {
    let cluster = client.create_cluster().cluster_name(name).send().await?;

    println!("cluster created: {:?}", cluster);

    Ok(())
}

async fn remove_cluster(client: &Client, name: &str) -> Result<(), Error> {
    let cluster_deleted = client.delete_cluster().cluster(name).send().await?;

    println!("cluster deleted: {:?}", cluster_deleted);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Opt {
        region,
        name,
        verbose,
    } = Opt::parse();

    if verbose {
        tracing_subscriber::fmt::init();
    }

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    make_cluster(&client, &name).await?;
    remove_cluster(&client, &name).await
}
