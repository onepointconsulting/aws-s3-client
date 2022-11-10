use aws_sdk_s3::model::{Delete, ObjectIdentifier};
use crate::OutputPrinter;
use crate::client_bucket::ClientBucket;

pub(crate) async fn delete_object(client_bucket: &ClientBucket,
                                  key: &str,
                                  output_printer: &dyn OutputPrinter) {
    let client = &client_bucket.client;
    let bucket_name = &client_bucket.bucket_name;

    let mut delete_objects: Vec<ObjectIdentifier> = vec![];
    let obj_id = ObjectIdentifier::builder()
        .set_key(Some(key.to_string()))
        .build();
    delete_objects.push(obj_id);

    let delete_res = client
        .delete_objects()
        .bucket(bucket_name)
        .delete(Delete::builder().set_objects(Some(delete_objects)).build())
        .send()
        .await;

    match delete_res {
        Ok(d) => {
            output_printer.ok_output(format!("Deleted successfully {}",
                                             key.to_string()).as_str());
        }
        Err(e) => {
            output_printer.ok_output(format!("Delete failed {:?}", e).as_str());
        }
    }
}