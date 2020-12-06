use std::{
    future::Future,
    pin::Pin,
    task:: {
        Context, 
        Poll
    },
    time::Instant,
    thread
};

pub struct Delay {
    pub when: Instant
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world!");
            Poll::Ready("done")
        } else {
            //get waker handle for current task
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}