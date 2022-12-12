use aws_client::{DefaultPrinter};

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::error::Error;
    use std::fmt;
    use aws_endpoint::BoxError;
    use aws_sdk_s3::error::DeleteBucketError;
    use aws_smithy_http::result::SdkError;
    use aws_client::cli::Cli;
    use aws_client::{OutputPrinter, print_message};
    use aws_client::ClientBucket;
    use aws_client::cli::Operation;
    use aws_client::check_print_env_variables;

    use super::*;

    #[derive(Debug)]
    struct DummyError {
    }

    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SuperError is here!")
        }
    }

    impl Error for DummyError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    fn output_printer_provider() -> DefaultPrinter {
        let output_printer = DefaultPrinter { sep: ",".to_string(), success: RefCell::new(0),
            error: RefCell::new(0) };
        output_printer
    }

    fn print_message_provider() -> (String, DefaultPrinter, String, String) {
        let bucket_name = &"test.bucket".to_string();
        let output_printer = output_printer_provider();
        let ok_message = "create";
        let error_message = "failed to create";
        (bucket_name.to_string(), output_printer, ok_message.to_string(), error_message.to_string())
    }

    #[test]
    fn when_print_message_should_have_success_1() {
        let res: Result<&str, SdkError<DeleteBucketError>> = Ok("OK");
        let (bucket_name, output_printer, ok_message, error_message) = print_message_provider();
        print_message(res, &bucket_name, &output_printer, ok_message.as_str(), error_message.as_str());
        assert_eq!(output_printer.success.take(), 1);
        assert_eq!(output_printer.error.take(), 0);
    }

    #[test]
    fn when_print_message_should_have_error_1() {
        let res: Result<&str, SdkError<DeleteBucketError>> = Err(SdkError::ConstructionFailure(Box::new(DummyError{})));
        let (bucket_name, output_printer, ok_message, error_message) = print_message_provider();
        print_message(res, &bucket_name, &output_printer, ok_message.as_str(), error_message.as_str());
        assert_eq!(output_printer.success.take(), 0);
        assert_eq!(output_printer.error.take(), 1);
    }

    #[test]
    fn when_create_bucket_should_have_success() {
        let cli = Cli {
            region: "eu-west-2".to_string(),
            glob_pattern: Some("*".to_string()),
            list_regex_pattern: Some(".+".to_string()),
            bucket: Some("gil.rust.test".to_string()),
            target_bucket: None,
            target_folder: None,
            mode: Operation::CreateBucket,
            sep: Some(",".to_string()),
            asc: None,
            source_key: None,
            target_key: None,
            flatten: false,
            strict_bucket: false
        };
        let output_printer = output_printer_provider();
        check_print_env_variables(&output_printer);
        // let client_bucket = ClientBucket {
        //     client: ,
        //     bucket_name: cli.bucket.unwrap().to_string(),
        //     args: cli
        // };
    }

}