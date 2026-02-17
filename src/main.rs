use std::{collections::VecDeque, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}};



struct Executor {
    queue: Mutex<VecDeque<Arc<Task>>>
}

impl Executor {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue:Mutex::new(VecDeque::new())
        })
    }

    fn spawn(self: &Arc<Self>, fut: impl Future<Output = ()> + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            executor: self.clone()
        });

        self.queue.lock().unwrap().push_back(task);
    }
    
    fn run(self: &Arc<Self>) {
        while let Some(task) = {
            let mut queue = self.queue.lock().unwrap();
            queue.pop_front()
        } {
            task.poll();
        }
    }
}


unsafe fn clone(data: *const ()) -> RawWaker {
    let arc = unsafe { Arc::from_raw(data as *const Task)};
    let cloned = arc.clone();
    std::mem::forget(arc);
    Task::raw_waker(cloned)
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()>>>>,
    executor: Arc<Executor>
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