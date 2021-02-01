use std::{
    future::Future,
    sync::Arc
};
use crossbeam::channel;
use super::task::Task;

pub struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>
}

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    pub fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        
        MiniTokio { scheduled, sender}
    }

    pub fn run (&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }

    pub fn spawn<F>(&mut self, future: F) 
    where F: Future<Output = ()> + Send + 'static, {
        Task::spawn(future, &self.sender);
    }
}