use std::cmp::Ordering;

use aws_sdk_s3::model::Object;

#[derive(Clone)]
pub(crate) struct ResultSorter {
    pub(crate) results: Vec<Object>,
    pub(crate) asc: i64
}

impl ResultSorter {
    pub(crate) fn sort_results(&mut self, obj: Object) {
        self.results.push(obj);
    }

    pub(crate) fn get_sorted(&mut self) -> Vec<Object> {
        let sorter = match self.asc {
            1 => |a: &Object, b: &Object|
                    a.last_modified().unwrap().secs().cmp(&b.last_modified().unwrap().secs()),
            -1 => |a: &Object, b: &Object|
                    b.last_modified().unwrap().secs().cmp(&a.last_modified().unwrap().secs()),
            _ => |_: &Object, _: &Object| Ordering::Equal
        };
        self.results.sort_by(sorter);
        let values: Vec<Object> = self.results.clone();
        values
    }
}
