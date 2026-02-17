use std::{ sync::Arc, task:: RawWaker};

use crate::task::task::Task;

pub unsafe fn clone(data: *const ()) -> RawWaker {
    let arc = unsafe { Arc::from_raw(data as *const Task)};
    let cloned = arc.clone();
    std::mem::forget(arc);
    Task::raw_waker(cloned)
}

pub unsafe fn wake(data: *const ()) {
    let arc = unsafe { Arc::from_raw(data as *const Task)};
    arc.schedule();
}

pub unsafe fn wake_by_ref(data: *const()) {
    let arc = unsafe { Arc::from_raw(data as *const Task)};
    arc.schedule();
    std::mem::forget(arc);
}

pub unsafe fn drop(data: *const ()) {
    let _ = unsafe { Arc::from_raw(data as *const Task)};
}
