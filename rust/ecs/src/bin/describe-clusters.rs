#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecs::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    // The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    // Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

async fn show_clusters(client: &Client) -> Result<(), Error> {
    let resp = client.list_clusters().send().await?;

    let cluster_arns = resp.cluster_arns().unwrap_or_default();
    println!("Found {} clusters", cluster_arns.len());

    let clusters = client
        .describe_clusters()
        .set_clusters(Some(cluster_arns.into()))
        .send()
        .await?;

    for cluster in clusters.clusters().unwrap_or_default() {
        println!("  ARN:    {}", cluster.cluster_arn().unwrap());
        println!("  Name:   {}", cluster.cluster_name().unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Opt { region, verbose } = Opt::parse();

    if verbose {
        tracing_subscriber::fmt::init();
    }

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    if verbose {
        println!();
        println!("ECS client version: {}", PKG_VERSION);
        println!(
            "Region:        {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_clusters(&client).await
}
