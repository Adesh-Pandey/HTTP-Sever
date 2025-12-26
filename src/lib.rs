// i want a thread pool implementation

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    id: String,
    thread: JoinHandle<()>,
}

enum Message {
    JobRequest(Job),
    TERMINATE,
}
impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> JoinHandle<()> {
        return thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::JobRequest(j) => j(),
                Message::TERMINATE => {
                    break;
                }
            }
        });
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    pub fn new(count: usize) -> ThreadPool {
        println!("NEw");
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut a = 0;
        let mut workers = vec![];
        loop {
            a = a + 1;
            workers.push(Worker {
                id: a.to_string(),
                thread: Worker::new(receiver.clone()),
            });
            if a == count {
                break;
            }
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let dyn_job = Box::new(job);
        self.sender.send(Message::JobRequest(dyn_job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.workers.iter().for_each(|worker| {
            println!("worker kill id:{}", worker.id);
            self.sender.send(Message::TERMINATE).unwrap();
        });

        // Prev iteration of this server had Option<JoinHandler<()>> but that is not necessary
        // as you can pop the workers vec to get ownership.
        while let Some(w) = self.workers.pop() {
            w.thread.join().unwrap();
        }
    }
}
