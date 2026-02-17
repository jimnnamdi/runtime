use crate::{counter::counter::{task_one, task_two}, executor::executor::Executor};


mod task;
mod waker;
mod counter;
mod executor;

fn main() {
    let ex = Executor::new();
    ex.spawn(task_one());
    ex.spawn(task_two());

    ex.run();
}