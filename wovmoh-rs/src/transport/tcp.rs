use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
};

use crate::{
    api::{
        self,
        request::Request,
        response::{Response, ResponseData},
    },
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
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn run(
    app: Arc<Mutex<App>>,
    bind_addr: SocketAddr,
) -> (JoinHandle<()>, mpsc::Sender<()>, SocketAddr) {
    let listener = TcpListener::bind(bind_addr).unwrap();
    let local_addr = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();

    let pool = ThreadPool::new(4);
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let shutting_down = Arc::new(AtomicBool::new(false));

    let join_handle = thread::spawn(move || {
        loop {
            if shutdown_rx.try_recv().is_ok() {
                shutting_down.store(true, Ordering::Release);
                break;
            }

            match listener.accept() {
                Ok((stream, _)) => {
                    let app = Arc::clone(&app);
                    let shutting_down = Arc::clone(&shutting_down);
                    pool.execute(move || handle_client(app, stream, shutting_down));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => {
                    eprintln!("TCP error {e}");
                    shutting_down.store(true, Ordering::Release);
                    break;
                }
            }
        }
    });

    (join_handle, shutdown_tx, local_addr)
}

fn handle_client(app: Arc<Mutex<App>>, stream: TcpStream, shutting_down: Arc<AtomicBool>) {
    let addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", addr);
    stream
        .set_read_timeout(Some(std::time::Duration::from_millis(100)))
        .unwrap();

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Client disconnected: {}", addr);
                break;
            }

            Ok(_) => match serde_json::from_str::<Request>(&line.replace("<null>", "null")) {
                Ok(request) => {
                    println!("[{}] {:?}", addr, request);
                    api::handler::handle_request(app.clone(), request, reader.get_mut());
                }
                Err(e) => {
                    eprintln!("[{}] Error deserializing request: {:?}", addr, e);
                    eprintln!("{}", &line.replace("<null>", "null"));

                    let value: serde_json::Value =
                        serde_json::from_str(&line.replace("<null>", "null"))
                            .unwrap_or(serde_json::Value::Null);

                    let value = value.get("id").cloned().unwrap_or(serde_json::Value::Null);
                    let id: u64 = match value.is_number() {
                        true => value.as_i64().unwrap() as u64,
                        false => u64::MAX,
                    };

                    let response = Response {
                        id: id,
                        data: Ok(ResponseData::Error {
                            message: e.to_string(),
                        }),
                    };

                    let mut json = serde_json::to_string(&response).unwrap();
                    json.push('\n');

                    reader.get_mut().write(json.as_bytes()).unwrap();
                }
            }, // Match serde::from_str

            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut
                {
                    if shutting_down.load(Ordering::Acquire) {
                        break;
                    }
                    continue;
                }
                eprintln!("[{}] Read error: {:?}", addr, e);
                break;
            }
        }
    }
}
