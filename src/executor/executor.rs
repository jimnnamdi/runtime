use std::{collections::VecDeque,sync::{Arc, Mutex}};

use crate::task::task::Task;

pub struct Executor {
    pub queue: Mutex<VecDeque<Arc<Task>>>
}

impl Executor {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue:Mutex::new(VecDeque::new())
        })
    }

    pub fn spawn(self: &Arc<Self>, fut: impl Future<Output = ()> + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            executor: self.clone()
        });

        self.queue.lock().unwrap().push_back(task);
    }
    
    pub fn run(self: &Arc<Self>) {
        while let Some(task) = {
            let mut queue = self.queue.lock().unwrap();
            queue.pop_front()
        } {
            task.poll();
        }
    }
}
