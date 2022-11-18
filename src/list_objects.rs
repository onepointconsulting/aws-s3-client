use std::future::Future;

use aws_sdk_s3::Client;
use aws_sdk_s3::Error;
use aws_sdk_s3::model::Object;
use fancy_regex::Regex;

use crate::{ClientBucket, OutputPrinter, Region, ResultSorter};

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
    let regex = match &client_bucket.args.list_regex_pattern {
        Some(re) => {
            re
        }
        None => {
            ".+"
        }
    };
    let asc = match &client_bucket.args.asc {
        Some(asc_bool) => {
            if *asc_bool { 1 } else { -1 }
        }
        None => 1
    };
    let mut result_sorter = ResultSorter { results: Vec::new(), asc };
    let re = &Regex::new(regex).expect("Invalid regex");
    for obj in objects.contents().unwrap_or_default() {
        let key_str = obj.key().unwrap();
        if find_regex(key_str, re) > -1 {
            result_sorter.sort_results(obj.clone());
        }
    }

    for obj in result_sorter.get_sorted().iter() {
        process_obj(client_bucket, obj.clone(), output_printer).await;
    }

    Ok(())
}

pub(crate) async fn list_buckets(client: &Client, output_printer: &dyn OutputPrinter, region_option: Option<Region>)
                                 -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    if region_option.is_some() {
        let region = region_option.unwrap();
        output_printer.ok_output("TBD");
    } else {
        output_printer.ok_output(format!("There are a total of {} buckets", num_buckets).as_str());
    }

    Ok(())
}