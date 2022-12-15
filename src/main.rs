extern crate alloc;
extern crate core;
extern crate glob;

use std::cell::RefCell;
use std::path::Path;

use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::model::Object;
use aws_smithy_http::byte_stream::ByteStream;
use clap::Parser;

use aws_client::{check_print_env_variables, DefaultPrinter, OutputPrinter};
use aws_client::cli::Cli;
use aws_client::cli::Operation;
use aws_client::ClientBucket;
use Operation::{CopyBucketToBucket, CopyMultiple, CopySingle, CreateBucket, Delete, DeleteBucket, Download, List,
                ListBuckets, MoveMultiple, MoveSingle, Upload, ListObjectVersions};

use crate::bucket_operations::{copy_to_bucket, create_bucket, delete_bucket, list_buckets};
use crate::client_factory::setup;
use crate::copy_operations::{copy_multiple_process_obj, copy_object, move_multiple_process_obj, move_object};
use crate::file_delete::delete_object;
use crate::file_download::download_object;
use crate::list_objects::{list_object_versions, list_objects};
use crate::result_sorter::ResultSorter;
use crate::upload_files::upload_files_operation;

mod cli;
mod output_printer;
mod result_sorter;
mod file_download;
mod file_delete;
mod list_objects;
mod copy_operations;
mod upload_files;
mod bucket_operations;
mod date_utils;
mod client_factory;

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
    let output_printer = DefaultPrinter {
        sep: sep.to_string(),
        success: RefCell::new(0),
        error: RefCell::new(0),
    };

    check_print_env_variables(&output_printer);

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
                if res.is_err() {
                    output_printer.err_output(format!("Could not list bucket: {:?}", res.err().unwrap()).as_str());
                }
            }
            ListObjectVersions => {
                let res = list_object_versions(client_bucket, &output_printer).await;
                if res.is_err() {
                    output_printer.err_output(format!("Could not list bucket versions: {:?}", res.err().unwrap()).as_str());
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
                let _ = create_bucket(client_bucket, &output_printer).await;
            }
            DeleteBucket => {
                let _ = delete_bucket(client_bucket, &output_printer).await;
            }
            CopyBucketToBucket => {
                copy_to_bucket(client_bucket,
                               &output_printer).await;
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
