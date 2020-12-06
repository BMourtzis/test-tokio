use std::time::{Duration, Instant};
use test_tokio::{
    delay::Delay,
    mini_tokio::MiniTokio
};

fn main() {
    let mut mt = MiniTokio::new();

    mt.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mt.run();
}