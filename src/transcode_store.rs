//use crate::transcode_task::TranscodeTask;

use crate::ffjob::FfJob;
use crate::transcode_task::TranscodeTask;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct TranscodeStore {
    taskStore: HashMap<FfJob, Arc<TranscodeTask>>
}

impl TranscodeStore {
    pub fn new() -> TranscodeStore {
        return  TranscodeStore {
            taskStore: HashMap::new()
        }
    }
    pub fn get_or_create_task(self: &mut TranscodeStore, job: FfJob) -> Arc<TranscodeTask> {
        let map = &mut self.taskStore;
        let opt = map.get(&job);
        if opt.is_none() {
            let t = Arc::new(TranscodeTask::new());
            map.insert(job, t.clone());
            return t;
        } else {
            return opt.unwrap().clone();
        }
    }

}

#[test]
fn test_store_concurrency() {
    let _store = TranscodeStore::new();
    let _task = crate::transcode_task::TranscodeTask::new();
}

