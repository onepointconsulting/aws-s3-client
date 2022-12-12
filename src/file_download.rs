use std::fs;
use std::path::PathBuf;
use aws_smithy_http::byte_stream::{AggregatedBytes};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use aws_client::OutputPrinter;

use aws_client::ClientBucket;

pub(crate) async fn download_object(client_bucket: &ClientBucket,
                                    key: &str,
                                    output_printer: &dyn OutputPrinter) {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let target_folder_str = &client_bucket.args.target_folder.as_ref().expect("Please define the target folder for download");
    let path: PathBuf = PathBuf::from(*target_folder_str);
    let flatten = &client_bucket.args.flatten;
    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await;
    match resp {
        Ok(obj) => {
            let stream = obj.body.collect().await;
            let splits = key.split("/");
            let last_split_option = splits.last();
            let last = last_split_option.unwrap();
            if last.len() > 0 {
                let new_path = if *flatten { path.join(PathBuf::from(last)) } else {
                    path.join(PathBuf::from(key)).join(PathBuf::from(last))
                };
                let parent_dir = new_path.parent().unwrap();
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir.clone())
                        .expect(format!("Could not create {:?}", parent_dir.clone()).as_str());
                }
                write_file(key, output_printer, &mut stream.unwrap(), new_path).await;
            }
        }
        Err(e) => {
            output_printer.err_output(format!("Cannot download {} due to {:?}", key, e).as_str());
        }
    }
}

async fn write_file(key: &str, output_printer: &dyn OutputPrinter, bytes: &mut AggregatedBytes, new_path: PathBuf) {
    let mut file = File::create(new_path.clone()).await.unwrap();
    let _ = file.write_all_buf(bytes).await;
    file.flush().await.expect("Failed to flush downloaded message");
    output_printer.ok_output(format!("Download successfully {} to {:?}", key, new_path).as_str());
}