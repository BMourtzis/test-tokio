use std::time::{
    Duration,
    Instant
};
use test_tokio::delay::Delay;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay {
        when
    };

    let out = future.await;
    assert_eq!(out, "done");
}