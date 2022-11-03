use aws_sdk_s3::Client;
use crate::Cli;

pub(crate) struct ClientBucket {
    pub(crate) client: Client,
    pub(crate) bucket_name: String,
    pub(crate) args: Cli,
}

impl ClientBucket {
    pub fn new(client: Client, bucket_name: String, args: Cli) -> ClientBucket {
        ClientBucket {
            client,
            bucket_name,
            args,
        }
    }
}