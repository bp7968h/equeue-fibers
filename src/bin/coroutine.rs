use equeue_fibers::coroutine::{async_main, future::{PollState, Future}};
use std::time::Duration;
use std::thread;

fn main() {
    let mut future = async_main();
    loop {
        match future.poll() {
            PollState::NotReady => {
                println!("Schedule other tasks");
            },
            PollState::Ready(_) => break,
        }
        thread::sleep(Duration::from_millis(100));
    }
}