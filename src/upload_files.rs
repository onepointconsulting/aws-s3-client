extern crate glob;

use std::path::Path;

use aws_sdk_s3::model::{CompletedMultipartUpload, CompletedPart};
use aws_smithy_http::byte_stream::{ByteStream, Length};
use simple_error::{SimpleError};

use aws_client::{ClientBucket, OutputPrinter};

use crate::upload_object;

use self::glob::glob;

const MAX_CHUNKS: u64 = 10000;


pub(crate) async fn upload_files_operation(glob_pattern: &String,
                                           client_bucket: &ClientBucket,
                                           output_printer: &dyn OutputPrinter) {
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


const DEFAULT_CHUNK_SIZE: u64 = 5;

pub(crate) async fn upload_file_in_chunks(client_bucket: &ClientBucket, output_printer: &dyn OutputPrinter) -> Result<(), SimpleError> {
    let bucket_name = &client_bucket.bucket_name;
    let chunk_size: u64 = client_bucket.args.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
    let file_name: &str = client_bucket.args.upload_file.as_ref().expect("Please specify the file name").as_str();
    let file_path = Path::new(file_name);
    let chunk_size_bytes = chunk_size * 1024 * 1024;
    if !file_path.exists() {
        return Err(SimpleError::new(format!("Cannot find file {}.", file_name.to_string()).as_str()));
    }
    let key = file_path.file_name().unwrap().to_str().unwrap();
    output_printer.ok_output(format!("Uploading to {}", bucket_name).as_str());

    let result = client_bucket.client
        .create_multipart_upload()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await;
    if result.is_err() {
        return Err(SimpleError::new(format!("Cannot start multi part upload {:?}.", result.err().unwrap()).as_str()));
    }
    let result_output = result.unwrap();
    let upload_id = result_output.upload_id().expect("Missing upload id");
    let file_size = tokio::fs::metadata(file_path).await
        .expect(format!("Cannot access file: {}", file_path.to_str().unwrap()).as_str()).len();

    if file_size == 0 {
        return Err(SimpleError::new(format!("File is empty {}.", file_name)));
    }

    let (chunk_count, size_of_last_chunk) = calculate_chunks(file_name, file_size,
                                                             chunk_size_bytes.clone());

    let mut upload_parts: Vec<CompletedPart> = Vec::new();
    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            chunk_size_bytes
        };
        let stream = ByteStream::read_from()
            .path(file_path)
            .offset(chunk_index * chunk_size_bytes)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();
        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client_bucket.client
            .upload_part()
            .key(key)
            .bucket(bucket_name)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await;
        upload_parts.push(CompletedPart::builder()
                              .e_tag(upload_part_res.unwrap().e_tag.unwrap_or_default())
                              .part_number(part_number)
                              .build(),
        )
    }

    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();

    let _complete_multipart_upload_res = client_bucket.client
        .complete_multipart_upload()
        .bucket(bucket_name)
        .key(key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();

    output_printer.ok_output(format!("File {} uploaded successfully", file_name).as_str());

    Ok(())
}

fn calculate_chunks(file_name: &str, file_size: u64, chunk_size_bytes: u64) -> (u64, u64) {
    let mut chunk_count = file_size / chunk_size_bytes + 1;
    let mut size_of_last_chunk = file_size % chunk_size_bytes;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = chunk_size_bytes;
        chunk_count -= 1;
    }
    if chunk_count > MAX_CHUNKS {
        panic!("Too many chunks {}.", file_name);
    }
    (chunk_count, size_of_last_chunk)
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_chunks() {
        let file_name = "text.mp3";
        let file_size_mb = 10;
        let file_size = (file_size_mb * 1024 * 1024) as u64;
        let chunk_size_mb = 2;
        let chunk_size_bytes = (chunk_size_mb * 1024 * 1024) as u64;
        let (chunk_count, _) = calculate_chunks(file_name, file_size, chunk_size_bytes);
        assert_eq!(chunk_count, file_size_mb / chunk_size_mb);
    }

}