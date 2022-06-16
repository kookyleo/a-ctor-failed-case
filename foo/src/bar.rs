use crate::COUNTER;
use std::sync::atomic::{Ordering, Ordering::SeqCst};
use ctor::ctor;

#[ctor]
fn init() {
    COUNTER.store(COUNTER.load(SeqCst) + 1, Ordering::SeqCst)
}
