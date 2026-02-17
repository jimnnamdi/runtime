use std::task::Poll;



pub struct Counter {
    pub c_val: i32,
    pub valid: bool
}

impl Future for Counter {
    type Output = i32;
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.c_val == 5 && self.valid {
            self.valid = true;
            Poll::Ready(self.c_val)
        } else {
            self.c_val += 1;
            self.valid = false;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub fn yield_now() -> Counter {
    Counter { c_val: 0, valid: false }
}

pub async fn task_one() {
    for i in 0..5 {
        println!("task one : {}", i);
        yield_now().await;
    }
}

pub async fn task_two() {
    for i in 0..5 {
        println!("task two : {}", i);
        yield_now().await;
    }
}