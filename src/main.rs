use std::{collections::VecDeque, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}};



struct Executor {
    queue: Mutex<VecDeque<Arc<Task>>>
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()>>>>,
    executor: Executor
}
static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

impl Task {
    fn raw_waker(task: Arc<Task>) -> RawWaker {
        RawWaker::new(Arc::into_raw(task) as *const (), &VTABLE)
    }

    fn create_waker(self: &Arc<Self>) -> Waker {
        unsafe { Waker::from_raw(Self::raw_waker(self.clone())) }
    }

    fn poll(self: &Arc<Task>) {
        let waker = self.clone().create_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = unsafe { Pin::new_unchecked(self.future.lock().unwrap())};

        if let Poll::Pending = future.as_mut().poll(&mut context) {}
    }

    fn schedule(self: &Arc<Self>) {
        self.executor.queue.lock().unwrap().push_back(self.clone());
    }
}

fn main() {}