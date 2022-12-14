

use date_utils::convert_date_time;
use std::cell::RefCell;
use aws_sdk_s3::model::Object;
use aws_sdk_s3::Client;
use cli::Cli;
use std::env;

mod date_utils;
pub mod cli;
pub mod client_factory;
pub mod bucket_operations;

pub trait OutputPrinter {
    fn output_with_stats(&self, obj: &Object);
    fn err_output(&self, msg: &str);
    fn ok_output(&self, msg: &str);
}

pub struct DefaultPrinter {
    pub sep: String,
    pub success: RefCell<u32>,
    pub error: RefCell<u32>
}

impl OutputPrinter for DefaultPrinter {
    fn output_with_stats(&self, obj: &Object) {
        let key_str = obj.key().unwrap();
        let size = obj.size();
        let last_modified = obj.last_modified();
        let d = convert_date_time(last_modified);
        println!("{}{}{:?}{}{} Kb", key_str, self.sep, d, self.sep, size / 1024);
    }

    fn err_output(&self, msg: &str) {
        eprintln!("{}", msg);
        self.error.replace(&self.error.take() + 1);
    }

    fn ok_output(&self, msg: &str) {
        println!("{}", msg);
        self.success.replace(&self.success.take() + 1);
    }
}

pub struct ClientBucket {
    pub client: Client,
    pub bucket_name: String,
    pub args: Cli,
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

pub fn check_print_env_variables(output_printer: &dyn OutputPrinter) {
    output_printer.ok_output(format!("AWS_ACCESS_KEY_ID: '{}'", env::var("AWS_ACCESS_KEY_ID")
        .expect("Please provide the ASW_ACCESS_KEY")).as_str());
    env::var("AWS_SECRET_ACCESS_KEY")
        .expect("Please provide the AWS_SECRET_ACCESS_KEY as environment variable");
    let result = "*********************************";
    output_printer.ok_output(format!("AWS_SECRET_ACCESS_KEY: {}", result).as_str());
    output_printer.ok_output("");
}


