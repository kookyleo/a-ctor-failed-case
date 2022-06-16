use ctor::ctor;
use foo::COUNTER;
use std::sync::atomic::{Ordering, Ordering::SeqCst};

#[ctor]
fn init() {
    COUNTER.store(COUNTER.load(SeqCst) + 1, Ordering::SeqCst)
}

fn main() {
    // It is going to print 2,
    // but the question is why not 3? obviously, the COUNTER is added once in each of the 3 rs files...
    println!("The value of COUNTER is {:?}.", COUNTER);
}
