use std::fs;
use tokio::fs::File;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use crate::{ClientBucket, OutputPrinter};

pub(crate) async fn download_object(client_bucket: &ClientBucket,
                                    key: &str,
                                    output_printer: & dyn OutputPrinter) {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let target_folder_str = &client_bucket.args.target_folder.as_ref().expect("Please define the target folder for download");
    let path: PathBuf = PathBuf::from(*target_folder_str);
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
                let new_path = path.join(PathBuf::from(last));
                if !path.exists() {
                    fs::create_dir_all(path.clone())
                        .expect(format!("Could not create {:?}", path.clone()).as_str());
                }
                let mut file = File::create(new_path.clone()).await.unwrap();
                let _ = file.write_all_buf(&mut stream.unwrap()).await;
                file.flush().await.expect("Failed to flush downloaded message");
                output_printer.ok_output(format!("Download successfully {} to {:?}", key, new_path).as_str());
            }
        }
        Err(e) => {
            output_printer.err_output(format!("Cannot download {} due to {:?}", key, e).as_str());
        }
    }
}