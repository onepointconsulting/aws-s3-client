use core::fmt::Debug;
use aws_sdk_s3::client::Client;
use core::result::Result;
use core::result::Result::Ok;
use aws_sdk_s3::Region;
use aws_sdk_s3::Error;
use aws_sdk_s3::error::DeleteBucketError;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::output::DeleteBucketOutput;
use aws_smithy_http::result::SdkError;
use crate::{ClientBucket, OutputPrinter};

pub(crate) async fn list_buckets(client: &Client,
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
        if strict {
            let r = client
                .get_bucket_location()
                .bucket(bucket.name().unwrap_or_default())
                .send()
                .await?;
            if r.location_constraint().unwrap().as_ref() == region_name {
                output_printer.ok_output(format!("{},{}",
                                                 bucket.name().unwrap_or_default(), region_name).as_str());
                in_region += 1;
            }
        } else {
            output_printer.ok_output(format!("{}", bucket.name().unwrap_or_default()).as_str());
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

pub(crate) async fn create_bucket(client_bucket: &ClientBucket,
                                  output_printer: &dyn OutputPrinter) {
    let client = &client_bucket.client;
    let region = &client_bucket.args.region.as_str();
    let bucket_name = &client_bucket.bucket_name;
    let constraint = BucketLocationConstraint::from(*region);
    let cfg = CreateBucketConfiguration::builder().location_constraint(constraint).build();
    let res = client.create_bucket().create_bucket_configuration(cfg).bucket(bucket_name).send().await;
    print_message(res, bucket_name, output_printer,
                  "created",
                  "An error occurred in create bucket");
}

pub(crate) async fn delete_bucket(client_bucket: &ClientBucket,
                                  output_printer: &dyn OutputPrinter) {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let res: Result<DeleteBucketOutput, SdkError<DeleteBucketError>> = client.delete_bucket().bucket(bucket_name).send().await;
    print_message(res, bucket_name, output_printer,
                  "deleted",
                  "An error occurred in delete bucket");
}

fn print_message<O, E>(res: Result<O, SdkError<E>>,
                       bucket_name: &String,
                       output_printer: &dyn OutputPrinter,
                       message1: &str,
                       message2: &str)
    where E: Debug {
    match res {
        Ok(_) => {
            output_printer.ok_output(format!("Bucket {} has been {}.", bucket_name, message1).as_str());
        }
        Err(e) => {
            output_printer.err_output(format!("{}: {:?}", message2, e).as_str());
        }
    }
}