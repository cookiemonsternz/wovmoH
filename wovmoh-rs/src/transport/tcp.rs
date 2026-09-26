use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use crate::{
    api::{self, request::Request},
    app::app::App,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn run(app: Arc<Mutex<App>>) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let app = Arc::clone(&app);

            pool.execute(move || handle_client(app, stream));
        }
    });
}

fn handle_client(app: Arc<Mutex<App>>, stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", addr);

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Client disconnected: {}", addr);
                break;
            }

            Ok(_) => match serde_json::from_str::<Request>(&line) {
                Ok(request) => {
                    println!("[{}] {:?}", addr, request);
                    api::handler::handle_request(app.clone(), request, reader.get_mut());
                }
                Err(e) => {
                    eprintln!("[{}] Invalid request: {:?}", addr, e);
                }
            }, // Match serde::from_str

            Err(e) => {
                eprintln!("[{}] Read error: {:?}", addr, e);
                break;
            }
        }
    }
}
