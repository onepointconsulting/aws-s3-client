extern crate glob;

use crate::{ClientBucket, OutputPrinter, upload_object};

use self::glob::glob;

pub(crate) async fn upload_files_operation(glob_pattern: &String,
                                           client_bucket: &ClientBucket,
                                           output_printer: & dyn OutputPrinter) {
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    let target_folder = &client_bucket.args.target_folder;
    let flatten = &client_bucket.args.flatten;
    let bucket_name = &client_bucket.bucket_name;
    match target_folder {
        Some(tf) => {
            for entry in glob(glob_pattern).expect(&expected) {
                match entry {
                    Ok(path) => {
                        let path_string = path.clone().into_os_string();
                        let file_name = if *flatten { path.file_name().unwrap() } else { path_string.as_os_str() };
                        let file_str = file_name.to_str().unwrap().replace("\\", "/");
                        let key = format!("{}/{}", tf, file_str);
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