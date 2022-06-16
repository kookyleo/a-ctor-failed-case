use ctor::ctor;
use std::sync::atomic::{AtomicI8, Ordering, Ordering::SeqCst};

pub static COUNTER: AtomicI8 = AtomicI8::new(0);

#[ctor]
fn init() {
    COUNTER.store(COUNTER.load(SeqCst) + 1, Ordering::SeqCst)
}

mod bar;

#[test]
fn test(){
    // It is going to print 2, this is as expected.
    println!("The value of COUNTER is {:?}.", COUNTER);
}
