extern crate alloc;
extern crate glob;

use std::collections::BTreeMap;
use std::env;
use std::future::Future;
use std::path::{Path};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error, Region};
use aws_sdk_s3::model::Object;
use aws_smithy_http::byte_stream::ByteStream;
use clap::Parser;
use fancy_regex::Regex;

use crate::cli::{Cli, Operation};
use crate::file_download::download_object;
use crate::output_printer::{DefaultPrinter, OutputPrinter};
use crate::result_sorter::ResultSorter;

use self::glob::glob;

mod cli;
mod output_printer;
mod result_sorter;
mod file_download;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let (_, client) = setup(&args).await;
    let mode = args.mode;
    let bucket = args.bucket.clone();
    println!("Bucket: {}", bucket);
    if env::var("AWS_ACCESS_KEY_ID").is_ok() {
        println!("AWS_ACCESS_KEY_ID: {}", env::var("AWS_ACCESS_KEY_ID").expect("Please provide the ASW_ACCESS_KEY"));
        println!("AWS_SECRET_ACCESS_KEY: {}", env::var("AWS_SECRET_ACCESS_KEY").expect("Please provide the AWS_SECRET_ACCESS_KEY"));
    }
    println!("");

    let default_sep = ",".to_string();
    let sep = match &args.sep {
        Some(s) => s,
        None => &default_sep
    };
    let output_printer = DefaultPrinter { sep: sep.to_string() };

    match mode {
        Operation::List => {
            async fn process_obj(_: &ClientBucket, obj: Object, output_printer: & dyn OutputPrinter) {
                output_printer.output_with_stats(&obj);
            }
            let res = list_objects(&ClientBucket {
                client,
                bucket_name: bucket,
                args,
            }, & output_printer, process_obj).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    println!("Could not list bucket: {}", e);
                }
            }
        }
        Operation::Upload => {
            let glob_pattern = &args.glob_pattern;
            match glob_pattern {
                Some(pattern) => {
                    upload_files(pattern, &client, &args).await;
                }
                None => {
                    println!("Error: please enter a glob pattern, like e.g: *.csv");
                }
            }
        }
        Operation::Download => {
            let client_bucket = &ClientBucket {
                client,
                bucket_name: bucket,
                args,
            };
            async fn process_obj(client_bucket: &ClientBucket,
                           obj: Object,
                           output_printer: &dyn OutputPrinter) {
                download_object(client_bucket, obj.key().unwrap(), output_printer).await
            }
            let _ = list_objects(client_bucket,
                                   &output_printer,
                                   process_obj).await;
        }
    }
}

pub(crate) struct ClientBucket {
    pub(crate) client: Client,
    pub(crate) bucket_name: String,
    pub(crate) args: Cli,
}

fn find_regex(content: &str, search_filter: &Regex) -> i32 {
    let result = search_filter.find(content);
    if result.is_ok() {
        let match_option = result.unwrap();
        match match_option {
            Some(m) => {
                return m.start() as i32;
            }
            None => {}
        }
    }
    return -1;
}

async fn upload_files(glob_pattern: &String, client: &Client, args: &Cli) {
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    let target_folder = &args.target_folder;
    let bucket_name = &args.bucket;
    match target_folder {
        Some(tf) => {
            for entry in glob(glob_pattern).expect(&expected) {
                match entry {
                    Ok(path) => {
                        let file_name = path.file_name().unwrap();
                        let key = format!("{}/{}", tf, file_name.to_str().unwrap());
                        let file_str = path.to_str().unwrap();
                        println!("Uploading {} to {}", file_str, key);
                        let res = upload_object(client, bucket_name.as_str(), file_str, key.as_str()).await;
                        match res {
                            Ok(_) => {
                                println!("Upload successful: {}", key);
                            }
                            Err(e) => {
                                println!("Could not upload: {}", e);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        None => {
            println!("Please specify the target folder");
        }
    }
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<(), Error> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await?;

    println!("Uploaded file: {}", file_name);
    Ok(())
}

async fn list_objects<'a, F, Fut>(client_bucket: &'a ClientBucket,
                              output_printer: &'a dyn OutputPrinter,
                              process_obj: F) -> Result<(), Error>
    where
        F: FnOnce(&'a ClientBucket, Object, &'a dyn OutputPrinter) -> Fut + std::marker::Copy,
        Fut: Future<Output=()>
{
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    println!("Objects in bucket:");
    let regex = match &client_bucket.args.list_regex_pattern {
        Some(re) => {
            re
        }
        None => {
            ".+"
        }
    };
    let asc = match &client_bucket.args.asc {
        Some(asc_bool) => {
            if *asc_bool { 1 } else { -1 }
        }
        None => 1
    };
    let mut result_sorter = ResultSorter { results: BTreeMap::new(), asc };
    let re = &Regex::new(regex).expect("Invalid regex");
    for obj in objects.contents().unwrap_or_default() {
        let key_str = obj.key().unwrap();
        if find_regex(key_str, re) > -1 {
            result_sorter.sort_results(obj.clone());
        }
    }

    for obj in result_sorter.get_sorted().iter() {
        process_obj(client_bucket, obj.clone(), output_printer).await;
    }

    Ok(())
}

async fn setup(args: &Cli) -> (Region, Client) {
    let region = &args.region;
    println!("Region: {}", region);
    let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()));
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);
    return (region, client);
}
