extern crate alloc;
extern crate glob;
extern crate core;

use std::env;
use std::path::{Path};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error, Region};
use aws_sdk_s3::model::Object;
use aws_smithy_http::byte_stream::ByteStream;
use clap::Parser;

use crate::cli::{Cli, Operation};
use crate::client_bucket::ClientBucket;
use crate::file_delete::delete_object;
use crate::file_download::download_object;
use crate::output_printer::{DefaultPrinter, OutputPrinter};
use crate::result_sorter::ResultSorter;
use crate::list_objects::list_objects;

use self::glob::glob;

mod cli;
mod output_printer;
mod result_sorter;
mod file_download;
mod client_bucket;
mod file_delete;
mod list_objects;

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
            async fn process_obj(_: &ClientBucket, obj: Object, output_printer: &dyn OutputPrinter) {
                output_printer.output_with_stats(&obj);
            }
            let client_bucket = &ClientBucket::new(client, bucket, args);
            let res = list_objects(client_bucket, &output_printer, process_obj).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    println!("Could not list bucket: {}", e);
                }
            }
        }
        Operation::Upload => {
            let glob_pattern = &args.glob_pattern.clone();
            let client_bucket = &ClientBucket::new(client, bucket, args);
            match glob_pattern {
                Some(pattern) => {
                    upload_files(pattern, client_bucket, &output_printer).await;
                }
                None => {
                    println!("Error: please enter a glob pattern, like e.g: *.csv");
                }
            }
        }
        Operation::Download => {
            let client_bucket = &ClientBucket::new(client, bucket, args);
            async fn process_obj(client_bucket: &ClientBucket,
                                 obj: Object,
                                 output_printer: &dyn OutputPrinter) {
                download_object(client_bucket, obj.key().unwrap(), output_printer).await
            }
            let _ = list_objects(client_bucket,
                                 &output_printer,
                                 process_obj).await;
        }
        Operation::Delete => {
            let client_bucket = &ClientBucket::new(client, bucket, args);
            async fn process_obj(client_bucket: &ClientBucket,
                                 obj: Object,
                                 output_printer: &dyn OutputPrinter) {
                delete_object(client_bucket, obj.key().unwrap(), output_printer).await
            }
            let _ = list_objects(client_bucket,
                                 &output_printer,
                                 process_obj).await;
        }
    }
}

async fn upload_files(glob_pattern: &String, client_bucket: &ClientBucket, output_printer: & dyn OutputPrinter) {
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    let target_folder = &client_bucket.args.target_folder;
    let bucket_name = &client_bucket.bucket_name;
    match target_folder {
        Some(tf) => {
            for entry in glob(glob_pattern).expect(&expected) {
                match entry {
                    Ok(path) => {
                        let file_name = path.file_name().unwrap();
                        let key = format!("{}/{}", tf, file_name.to_str().unwrap());
                        let file_str = path.to_str().unwrap();
                        output_printer.ok_output(format!("Uploading {} to {}", file_str, key).as_str());
                        let res = upload_object(&client_bucket.client,
                                                bucket_name.as_str(),
                                                file_str, key.as_str()).await;
                        match res {
                            Ok(_) => {
                                output_printer.ok_output(format!("Upload successful: {}", key).as_str());
                            }
                            Err(e) => {
                                output_printer.err_output(format!("Could not upload: {}", e).as_str());
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        None => {
            output_printer.err_output("Please specify the target folder");
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

async fn setup(args: &Cli) -> (Region, Client) {
    let region = &args.region;
    println!("Region: {}", region);
    let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()));
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);
    return (region, client);
}
