use std::cell::RefCell;
use aws_sdk_s3::model::Object;
use crate::date_utils::convert_date_time;

pub(crate) trait OutputPrinter {
    fn output_with_stats(&self, obj: &Object);
    fn err_output(&self, msg: &str);
    fn ok_output(&self, msg: &str);
}

pub(crate) struct DefaultPrinter {
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