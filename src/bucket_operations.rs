use core::result::Result;
use core::result::Result::Ok;
use std::fmt::Debug;

use aws_sdk_s3::client::Client;
use aws_sdk_s3::Error;
use aws_sdk_s3::error::{CreateBucketError, DeleteBucketError};
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::output::{CreateBucketOutput, DeleteBucketOutput};
use aws_sdk_s3::Region;
use aws_smithy_http::result::SdkError;

use crate::{ClientBucket, OutputPrinter};
use crate::date_utils::convert_date_time;

pub async fn list_buckets(client: &Client,
                          output_printer: &dyn OutputPrinter,
                          region: Region,
                          strict: bool)
                          -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let region_name = region.as_ref();
    let mut in_region = 0;
    let num_buckets = buckets.len();

    for bucket in buckets {
        let creation_date = convert_date_time(bucket.creation_date());
        if strict {
            let r = client
                .get_bucket_location()
                .bucket(bucket.name().unwrap_or_default())
                .send()
                .await?;
            if r.location_constraint().unwrap().as_ref() == region_name {
                output_printer.ok_output(format!("{},{},{}",
                                                 bucket.name().unwrap_or_default(),
                                                 region_name,
                                                 creation_date).as_str());
                in_region += 1;
            }
        } else {
            output_printer.ok_output(format!("{},{}",
                                             bucket.name().unwrap_or_default(),
                                             creation_date).as_str());
        }
    }

    if strict {
        output_printer.ok_output(
            format!("Found {} buckets in the {} region out of a total of {} buckets.",
                    in_region, region, num_buckets).as_str()
        );
    } else {
        output_printer.ok_output(format!("There are a total of {} buckets", num_buckets).as_str());
    }

    Ok(())
}

pub async fn create_bucket(client_bucket: &ClientBucket,
                           output_printer: &dyn OutputPrinter) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let client = &client_bucket.client;
    let region = &client_bucket.args.region.as_str();
    let bucket_name = &client_bucket.bucket_name;
    let constraint = BucketLocationConstraint::from(*region);
    let cfg = CreateBucketConfiguration::builder().location_constraint(constraint).build();
    let res = client.create_bucket().create_bucket_configuration(cfg)
        .bucket(bucket_name).send().await;
    print_message(&res, bucket_name, output_printer,
                  "created",
                  "An error occurred in create bucket");
    res
}

pub async fn delete_bucket(client_bucket: &ClientBucket,
                           output_printer: &dyn OutputPrinter) -> Result<DeleteBucketOutput, SdkError<DeleteBucketError>> {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let res: Result<DeleteBucketOutput, SdkError<DeleteBucketError>> = client.delete_bucket().bucket(bucket_name).send().await;
    print_message(&res, bucket_name, output_printer,
                  "deleted",
                  "An error occurred in delete bucket");
    res
}

pub async fn copy_to_bucket(client_bucket: &ClientBucket,
                            output_printer: &dyn OutputPrinter) {
    let source_bucket = &client_bucket.args.bucket.as_ref()
        .expect("Source bucket is missing. Please specify the source bucket.");
    let target_bucket = &client_bucket.args.target_bucket.as_ref()
        .expect("Target bucket is missing. Please specify the target bucket.");
    let source_key = &client_bucket.args.source_key.as_ref()
        .expect("Source key is missing. Please specify the source key.");
    let target_key = &client_bucket.args.target_key.as_ref()
        .expect("Target key is missing. Please specify the target key.");
    let client = &client_bucket.client;
    let mut source_bucket_and_object = "".to_string();
    source_bucket_and_object += source_bucket;
    source_bucket_and_object += "/";
    source_bucket_and_object += source_key;
    let res = client
        .copy_object()
        .copy_source(source_bucket_and_object.clone())
        .bucket(*target_bucket)
        .key(*target_key)
        .send()
        .await;
    match res {
        Ok(_) => {
            output_printer.ok_output(
                format!("Copied {} to {}/{}", source_bucket_and_object, target_bucket, target_key).as_str());
        }
        Err(e) => {
            output_printer.err_output(
                format!("Failed to copy {} to {}/{}", source_bucket_and_object, target_bucket, target_key).as_str());
            output_printer.err_output(
                format!("Error: {:?}", e).as_str());
        }
    }
}

pub fn print_message<O, E>(res: &Result<O, SdkError<E>>,
                           bucket_name: &String,
                           output_printer: &dyn OutputPrinter,
                           ok_message: &str,
                           error_message: &str)
    where E: Debug {
    match res {
        Ok(_) => {
            output_printer.ok_output(format!("Bucket {} has been {}.", bucket_name, ok_message).as_str());
        }
        Err(e) => {
            output_printer.err_output(format!("{}: {:?}", error_message, e).as_str());
        }
    }
}
