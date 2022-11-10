use aws_sdk_s3::Error;
use crate::{ClientBucket, delete_object, OutputPrinter};

pub(crate) async fn copy_object(
    client_bucket: &ClientBucket,
    output_printer: &dyn OutputPrinter
) -> Result<(), Error> {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let args = &client_bucket.args;
    let source_key = match &args.source_key {
        Some(s) => {
            s
        }
        None => {
            let msg = "Source key is missing. Please specify the source key";
            output_printer.err_output(msg);
            panic!("{}", msg.to_string())
        }
    };
    let target_key = match &args.target_key {
        Some(s) => {
            s
        }
        None => {
            let msg = "Target key is missing. Please specify the target key";
            output_printer.err_output(msg);
            panic!("{}", msg.to_string());
        }
    };

    let source_bucket_and_object = format!("{}/{}", bucket_name, source_key);

    client
        .copy_object()
        .copy_source(source_bucket_and_object)
        .bucket(bucket_name)
        .key(target_key)
        .send()
        .await?;

    output_printer.ok_output(format!("Successfully copied {} to {}", source_key, target_key).as_str());

    Ok(())
}

pub(crate) async fn move_object(
    client_bucket: &ClientBucket,
    output_printer: &dyn OutputPrinter
) -> Result<(), Error> {
    let args = &client_bucket.args;
    match copy_object(client_bucket, output_printer).await {
        Result::Ok(()) => {
            let source_key = &args.source_key.as_ref().unwrap();
            delete_object(&client_bucket, source_key, output_printer).await;
        }
        Result::Err(e) => {
            let source_key = &args.source_key.as_ref().unwrap();
            output_printer.err_output(format!("Failed to copy {}: {:?}", source_key, e).as_str())
        }
    }
    Ok(())
}