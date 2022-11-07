use aws_sdk_s3::Error;
use crate::{ClientBucket, OutputPrinter};

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