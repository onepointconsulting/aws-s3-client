use std::collections::BTreeMap;
use aws_sdk_s3::model::Object;

pub(crate) struct ResultSorter {
    pub(crate) results: BTreeMap<i64, Object>,
    pub(crate) asc: i64
}

impl ResultSorter {
    pub(crate) fn sort_results(&mut self, obj: Object) {
        let last_modified = obj.last_modified();
        match last_modified {
            Some(dt) => {
                self.results.insert(dt.secs() * self.asc, obj);
            }
            None => {}
        }
    }

    pub(crate) fn get_sorted(&mut self) -> Vec<Object> {
        let values: Vec<Object> = self.results.clone().into_values().collect();
        values
    }
}
