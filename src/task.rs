use std::{
    sync::{Arc, Mutex},
    pin::Pin,
    future::Future,
    task::Context,
};
use crossbeam::channel;
use futures::task::{
    self,
    ArcWake
};

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Self>>
}

impl Task {
    pub fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    pub fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }

    pub fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where F: Future<Output =()> + Send + 'static, 
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone()
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}