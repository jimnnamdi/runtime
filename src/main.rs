use std::{collections::VecDeque, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}};

mod task;
mod waker;
mod executor;


fn main() {}