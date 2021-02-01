use tokio::sync::oneshot;
use std::{
    future::Future,
    pin::Pin,
    task::{
        Context, Poll
    }
};

pub struct MySelect {
    pub rx1: oneshot::Receiver<&'static str>,
    pub rx2: oneshot::Receiver<&'static str>
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}