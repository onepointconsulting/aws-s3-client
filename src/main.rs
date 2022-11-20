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
use Operation::{MoveSingle, CopySingle, MoveMultiple, CopyMultiple, Delete, Download, Upload, List,
                ListBuckets, CreateBucket, DeleteBucket};
use crate::bucket_operations::{create_bucket, delete_bucket, list_buckets};

use crate::cli::{Cli, Operation};
use crate::client_bucket::ClientBucket;
use crate::copy_operations::{copy_multiple_process_obj, copy_object, move_multiple_process_obj, move_object};
use crate::file_delete::delete_object;
use crate::file_download::download_object;
use crate::output_printer::{DefaultPrinter, OutputPrinter};
use crate::result_sorter::ResultSorter;
use crate::list_objects::{list_objects};
use crate::upload_files::upload_files_operation;

mod cli;
mod output_printer;
mod result_sorter;
mod file_download;
mod client_bucket;
mod file_delete;
mod list_objects;
mod copy_operations;
mod upload_files;
mod bucket_operations;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let (region, client) = setup(&args).await;
    let mode = args.mode;
    let bucket_option = args.bucket.clone();
    let bucket_exists = !bucket_option.is_none();

    let default_sep = ",".to_string();
    let sep = match &args.sep {
        Some(s) => s,
        None => &default_sep
    };
    let output_printer = DefaultPrinter { sep: sep.to_string() };

    if env::var("AWS_ACCESS_KEY_ID").is_ok() {
        output_printer.ok_output(format!("AWS_ACCESS_KEY_ID: {}", env::var("AWS_ACCESS_KEY_ID")
            .expect("Please provide the ASW_ACCESS_KEY")).as_str());
        env::var("AWS_SECRET_ACCESS_KEY")
            .expect("Please provide the AWS_SECRET_ACCESS_KEY as environment variable");
        let result = "*********************************";
        output_printer.ok_output(format!("AWS_SECRET_ACCESS_KEY: {}", result).as_str());
        output_printer.ok_output("");
    }

    if bucket_exists {
        let bucket = bucket_option.unwrap();

        output_printer.ok_output(format!("Bucket: {}", bucket).as_str());
        output_printer.ok_output("");

        let client_bucket = &ClientBucket::new(client, bucket, args.clone());
        match mode {
            List => {
                async fn process_obj(_: &ClientBucket, obj: Object, output_printer: &dyn OutputPrinter) {
                    output_printer.output_with_stats(&obj);
                }

                let res = list_objects(client_bucket, &output_printer, process_obj).await;
                match res {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Could not list bucket: {}", e);
                    }
                }
            }
            Upload => {
                let glob_pattern = &args.glob_pattern.clone();
                match glob_pattern {
                    Some(pattern) => {
                        upload_files_operation(pattern, client_bucket, &output_printer).await;
                    }
                    None => {
                        println!("Error: please enter a glob pattern, like e.g: *.csv");
                    }
                }
            }
            Download => {
                async fn process_obj(client_bucket: &ClientBucket,
                                     obj: Object,
                                     output_printer: &dyn OutputPrinter) {
                    download_object(client_bucket, obj.key().unwrap(), output_printer).await
                }
                let _ = list_objects(client_bucket,
                                     &output_printer,
                                     process_obj).await;
            }
            Delete => {
                async fn process_obj(client_bucket: &ClientBucket,
                                     obj: Object,
                                     output_printer: &dyn OutputPrinter) {
                    delete_object(client_bucket, obj.key().unwrap(), output_printer).await
                }
                let _ = list_objects(client_bucket,
                                     &output_printer,
                                     process_obj).await;
            }
            CopyMultiple => {
                let _ = list_objects(client_bucket,
                                     &output_printer,
                                     copy_multiple_process_obj).await;
            }
            MoveMultiple => {
                let _ = list_objects(client_bucket,
                                     &output_printer,
                                     move_multiple_process_obj).await;
            }
            CopySingle => {
                let _ = copy_object(client_bucket, &output_printer).await;
            }
            MoveSingle => {
                let _ = move_object(client_bucket, &output_printer).await;
            }
            CreateBucket => {
                create_bucket(client_bucket, &output_printer).await;
            }
            DeleteBucket => {
                delete_bucket(client_bucket, &output_printer).await;
            }
            _ => {}
        }
    } else {
        if let ListBuckets = mode {
            let res = list_buckets(&client, &output_printer, region, args.strict_bucket).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    output_printer.err_output(format!("Failed to list buckets {:?}", e).as_str());
                }
            }
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
    let region_provider = RegionProviderChain::first_try(Region::new(region.to_string()));
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);
    return (region, client);
}
