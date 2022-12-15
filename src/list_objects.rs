use std::future::Future;

use aws_sdk_s3::Error;
use aws_sdk_s3::Error::Unhandled;
use aws_sdk_s3::model::Object;
use fancy_regex::Regex;

use aws_client::{ClientBucket, OutputPrinter};

use crate::ResultSorter;

fn find_regex(content: &str, search_filter: &Regex) -> i32 {
    let result = search_filter.find(content);
    if result.is_ok() {
        let match_option = result.unwrap();
        match match_option {
            Some(m) => {
                return m.start() as i32;
            }
            None => {}
        }
    }
    return -1;
}

pub(crate) async fn list_objects<'a, F, Fut>(client_bucket: &'a ClientBucket,
                                             output_printer: &'a dyn OutputPrinter,
                                             process_obj: F) -> Result<(), Error>
    where
        F: FnOnce(&'a ClientBucket, Object, &'a dyn OutputPrinter) -> Fut + std::marker::Copy,
        Fut: Future<Output=()>
{
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;
    let objects = client.list_objects_v2().bucket(bucket_name).send().await?;
    let re = extract_list_regex_pattern(&client_bucket);
    let asc = match &client_bucket.args.asc {
        Some(asc_bool) => {
            if *asc_bool { 1 } else { -1 }
        }
        None => 1
    };
    let mut result_sorter = ResultSorter { results: Vec::new(), asc };
    for obj in objects.contents().unwrap_or_default() {
        let key_str = obj.key().unwrap();
        if find_regex(key_str, &re) > -1 {
            result_sorter.sort_results(obj.clone());
        }
    }

    for obj in result_sorter.get_sorted().iter() {
        process_obj(client_bucket, obj.clone(), output_printer).await;
    }

    Ok(())
}

fn extract_list_regex_pattern(client_bucket: &&ClientBucket) -> Regex {
    let regex = match &client_bucket.args.list_regex_pattern {
        Some(re) => {
            re
        }
        None => {
            ".+"
        }
    };
    let re = Regex::new(&regex).expect("Invalid regex");
    return re.clone()
}

pub(crate) async fn list_object_versions(client_bucket: &ClientBucket,
                                         output_printer: &dyn OutputPrinter) -> Result<(), Error> {
    let client = &client_bucket.client;
    let bucket = &client_bucket.bucket_name;
    let re = extract_list_regex_pattern(&client_bucket);
    let res = client.list_object_versions().bucket(bucket.as_str()).send().await;
    match res {
        Ok(list) => {
            for version in list.versions().unwrap_or_default() {
                let key_str = version.key().unwrap_or_default();
                if find_regex(key_str, &re) > -1 {
                    output_printer.ok_output(format!("{} :: version ID: {}",
                                                     key_str,
                                                     version.version_id().unwrap_or_default()).as_str())
                }
            }
            return Ok(());
        }
        Err(e) => {
            return Err(Unhandled(e.into()));
        }
    }
}