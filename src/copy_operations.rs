use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::model::Object;
use crate::{Cli, ClientBucket, delete_object, OutputPrinter};

pub(crate) async fn copy_object(
    client_bucket: &ClientBucket,
    output_printer: &dyn OutputPrinter,
) -> Result<(), Error> {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let args = &client_bucket.args;
    let source_key = extract_source_key(output_printer, args);

    let target_key = extract_target_key(output_printer, args);

    copy_from_key_to_target(client, bucket_name, &source_key, &target_key).await?;

    output_printer.ok_output(format!("Successfully copied {} to {}", source_key, target_key).as_str());

    Ok(())
}

pub(crate) fn extract_source_key(output_printer: &dyn OutputPrinter, args: &Cli) -> String {
    let source_key = match &args.source_key {
        Some(s) => { s }
        None => {
            let msg = "Source key is missing. Please specify the source key";
            output_printer.err_output(msg);
            panic!("{}", msg.to_string())
        }
    };
    source_key.clone()
}

pub(crate) async fn copy_from_key_to_target(client: &Client,
                                            bucket_name: &String,
                                            source_key: &String,
                                            target_key: &String) -> Result<(), Error> {
    let source_bucket_and_object = format!("{}/{}", bucket_name, source_key);

    client
        .copy_object()
        .copy_source(source_bucket_and_object)
        .bucket(bucket_name)
        .key(target_key)
        .send()
        .await?;

    Ok(())
}

pub(crate) fn extract_target_key(output_printer: &dyn OutputPrinter, args: &Cli) -> String {
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
    target_key.clone()
}

pub(crate) async fn move_object(
    client_bucket: &ClientBucket,
    output_printer: &dyn OutputPrinter,
) -> Result<(), Error> {
    let args = &client_bucket.args;
    match copy_object(client_bucket, output_printer).await {
        Ok(()) => {
            let source_key = &args.source_key.as_ref().unwrap();
            delete_object(&client_bucket, source_key, output_printer).await;
        }
        Err(e) => {
            let source_key = &args.source_key.as_ref().unwrap();
            output_printer.err_output(format!("Failed to copy {}: {:?}", source_key, e).as_str())
        }
    }
    Ok(())
}

pub(crate) async fn copy_multiple_process_obj(
    client_bucket: &ClientBucket,
    obj: Object,
    output_printer: &dyn OutputPrinter,
) {
    let (source_key, target_key) = extract_source_target_keys(&client_bucket, obj, output_printer);
    let res = copy_from_key_to_target(&client_bucket.client, &client_bucket.bucket_name,
                                      &source_key, &target_key).await;
    match res {
        Ok(_) => {
            output_printer.ok_output(format!("Copied {} to {}",
                                             source_key, target_key).as_str());
        }
        Err(e) => {
            handle_copy_error(output_printer, source_key, target_key, e);
        }
    }
}

fn handle_copy_error(output_printer: &dyn OutputPrinter, source_key: String, target_key: String, e: Error) {
    output_printer.err_output(format!("Failed to copy {} to {}",
                                      source_key, target_key).as_str());
    output_printer.err_output(format!("Error {:?}", e).as_str());
}

fn extract_source_target_keys(client_bucket: &&ClientBucket, obj: Object, output_printer: &dyn OutputPrinter) -> (String, String) {
    let target_key_folder = extract_target_key(output_printer, &client_bucket.args);
    let source_key = obj.key().unwrap().to_string();
    let source_key_file = source_key.split("/").last().unwrap();
    let target_key = format!("{}/{}", target_key_folder, source_key_file);
    (source_key, target_key)
}

pub(crate) async fn move_multiple_process_obj(
    client_bucket: &ClientBucket,
    obj: Object,
    output_printer: &dyn OutputPrinter,
) {
    let (source_key, target_key) = extract_source_target_keys(&client_bucket, obj, output_printer);
    let res = copy_from_key_to_target(&client_bucket.client, &client_bucket.bucket_name,
                                      &source_key, &target_key).await;
    match res {
        Ok(_) => {
            output_printer.ok_output(format!("Copied {} to {}",
                                             source_key, target_key).as_str());
            delete_object(&client_bucket, source_key.as_str(), output_printer).await;
        }
        Err(e) => {
            handle_copy_error(output_printer, source_key, target_key, e);
        }
    }
}