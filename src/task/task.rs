
use std::{pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}};

use crate::{executor::executor::Executor, waker::waker::{clone, drop, wake, wake_by_ref}};

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()>>>>,
    pub executor: Arc<Executor>
}

pub static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

impl Task {
    pub fn raw_waker(task: Arc<Task>) -> RawWaker {
        RawWaker::new(Arc::into_raw(task) as *const (), &VTABLE)
    }

    pub fn create_waker(self: &Arc<Self>) -> Waker {
        unsafe { Waker::from_raw(Self::raw_waker(self.clone())) }
    }

    pub fn poll(self: &Arc<Task>) {
        let waker = self.clone().create_waker();
        let mut context = Context::from_waker(&waker);
        let mut future = unsafe { Pin::new_unchecked(self.future.lock().unwrap())};

        if let Poll::Pending = future.as_mut().poll(&mut context) {}
    }

    pub fn schedule(self: &Arc<Self>) {
        self.executor.queue.lock().unwrap().push_back(self.clone());
    }
}