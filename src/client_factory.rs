use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Region};
use crate::Cli;


pub async fn setup(args: &Cli) -> (Region, Client) {
    let region = &args.region;
    let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()));
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);
    return (region, client);
}