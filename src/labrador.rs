use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::ops::DerefMut;

use futures::sync::oneshot;
use futures::future;
use futures::Future;

use hyper;
use hyper_tls;
use hyper::{Response, Request, Error};

use tokio_core::reactor::Core;

type RequestResponsePair = (oneshot::Receiver<Request>, oneshot::Sender<Result<Response, Error>>);

// Client
#[derive(Debug)]
pub struct Client {
    tx: SyncSender<(oneshot::Receiver<Request>, oneshot::Sender<Result<Response, Error>>)>
}

impl Client {
    pub fn new(concurrent_requests: usize, ssl: bool, mut rate: Option<f32>) -> Self {
        let (tx, rx) = sync_channel::<RequestResponsePair>(concurrent_requests);

        if let Some(ref mut val) = rate {
            if *val < 0f32 {
                *val = 0f32;
            }
        }

        fn handle_requests<T>(mut core: Core, client: hyper::Client<T>, rx: Receiver<RequestResponsePair>, rate: Option<f32>)
            where T: hyper::client::Connect {
            let mut vec: Vec<Instant> = Vec::new();


            for (req_rx, res_tx) in rx {
                if let Some(rate) = rate {
                    let expired = Instant::now() - Duration::from_millis(((1f32 / rate) as u64 + 1).max(1) * 1000);
                    vec.retain(|instant| {
                        *instant >= expired
                    });

                    if vec.len() >= (rate as usize).max(1) {
                        thread::sleep(*vec.get(0).unwrap() + Duration::from_millis(((1f32 / rate) as u64 + 1).max(1) * 1000) - Instant::now());
                    }
                }

                if let Ok(request) = req_rx.wait() {
                    let future = client.request(request);

                    res_tx.send(core.run(future));
                    vec.push(Instant::now());
                }
            }
        }

        thread::spawn(move || {
            let core = Core::new().unwrap();
            if ssl {
                let client = hyper::Client::configure()
                    .connector(hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
                    .build(&core.handle());
                handle_requests(core, client, rx, rate);
            } else {
                let client = hyper::Client::new(&core.handle());
                handle_requests(core, client, rx, rate);
            }
        });

        Client { tx }
    }

    pub fn execute(&self, req: Request) -> oneshot::Receiver<Result<Response, Error>> {
        let (req_tx, req_rx) = oneshot::channel::<Request>();
        let (res_tx, res_rx) = oneshot::channel::<Result<Response, Error>>();

        self.tx.send((req_rx, res_tx)).ok();

        req_tx.send(req).ok();

        res_rx
    }
}

impl Default for Client {
    fn default() -> Self {
        ClientBuilder::default().build()
    }
}

// Client Builder
#[derive(Debug)]
pub struct ClientBuilder {
    concurrent_requests: usize,
    ssl: bool,
    rate: Option<f32>
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder::default()
    }

    pub fn concurrent_requests(mut self, size: usize) -> Self {
        self.concurrent_requests = size;
        self
    }

    pub fn ssl(mut self, ssl: bool) -> Self {
        self.ssl = ssl;
        self
    }

    pub fn rate(mut self, rate: Option<f32>) -> Self {
        self.rate = rate;
        self
    }

    pub fn build(self) -> Client {
        Client::new(self.concurrent_requests, self.ssl, self.rate)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        ClientBuilder {
            concurrent_requests: 256,
            ssl: false,
            rate: None
        }
    }
}