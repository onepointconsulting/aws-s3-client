use aws_sdk_s3::model::Object;
use aws_sdk_s3::types::DateTime;
use chrono::NaiveDateTime;

pub(crate) trait OutputPrinter {
    fn output_with_stats(&mut self, obj: &Object);
}

pub(crate) struct DefaultPrinter {
    pub sep: String
}

impl OutputPrinter for DefaultPrinter {
    fn output_with_stats(&mut self, obj: &Object) {
        let key_str = obj.key().unwrap();
        let size = obj.size();
        let last_modified = obj.last_modified();
        let last_modified_date_time: &DateTime = last_modified.unwrap();
        let d = NaiveDateTime::from_timestamp(last_modified_date_time.secs(), 0);
        println!("{}{}{:?}{}{} Kb", key_str, self.sep, d, self.sep, size / 1024);
    }
}