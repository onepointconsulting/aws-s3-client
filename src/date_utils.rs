use aws_sdk_s3::types::DateTime;
use chrono::NaiveDateTime;

pub(crate) fn convert_date_time(date_time_opt: Option<&DateTime>) -> NaiveDateTime {
    let date_time: &DateTime = date_time_opt.unwrap();
    NaiveDateTime::from_timestamp(date_time.secs(), 0)
}