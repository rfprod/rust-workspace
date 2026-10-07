//! Load balancer module.
//!
//! A simple load balancer that uses the round-robin algorithm for request processing.
//!
//! # To test that it actually works
//!
//! #### 1. pull a web server container image, e.g. `nginx`
//!
//! ```bash
//! sudo docker pull nginx
//! ```
//!
//! #### 2. start three containers in the interactive mode in three terminals
//!
//! 2.1 terminal 1
//!
//! ```bash
//! sudo docker run -it -p 8080:80 --name nginx1 nginx
//! ```
//!
//! 2.2 terminal 2
//!
//! ```bash
//! sudo docker run -it -p 8081:80 --name nginx2 nginx
//! ```
//!
//! 2.3 terminal 3
//!
//! ```bash
//! sudo docker run -it -p 8082:80 --name nginx3 nginx
//! ```
//!
//! #### 3. start the load balancer in another teminal - terminal 4
//!
//! ```bash
//! cargo run 6
//! ```
//!
//! #### 4. send requests to the load balancer's port from another terminal - terminal 5
//!
//! ```bash
//! curl http://localhost:8079
//! ```

use std::sync::{atomic::AtomicUsize, Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
pub async fn main() {
    let mut program = LoadBalancer::new().await;
    program.start().await;
}

struct LoadBalancer {
    backends: Arc<Vec<String>>,
    counter: Arc<AtomicUsize>,
    listener: TcpListener,
}

impl LoadBalancer {
    async fn new() -> LoadBalancer {
        let backend_addresses = vec![
            "127.0.0.1:8079".to_string(),
            "127.0.0.1:8080".to_string(),
            "127.0.0.1:8081".to_string(),
            "127.0.0.1:8082".to_string(),
        ];

        let backends = Arc::new(backend_addresses.clone());
        let counter = Arc::new(AtomicUsize::new(0));

        let address = &backend_addresses[0];
        let listener = match TcpListener::bind(address.clone()).await {
            Ok(result) => {
                println!("Load balancer listening on {}", address);
                result
            }
            Err(err) => {
                panic!(
                    "Failed to bind TCP Listener to the backend {}: {}",
                    address, err
                );
            }
        };

        LoadBalancer {
            backends,
            counter,
            listener,
        }
    }

    async fn start(&mut self) {
        loop {
            let (client_socket, _) = match self.listener.accept().await {
                Ok((socker, address)) => (socker, address),
                Err(err) => {
                    eprintln!(
                        "Failed to accept an incoming connection from the listener: {}",
                        err
                    );
                    return;
                }
            };

            let backends = Arc::clone(&self.backends);
            let counter = Arc::clone(&self.counter);

            tokio::spawn(async move {
                Self::handle_connection(client_socket, backends, counter).await;
            });
        }
    }

    async fn handle_connection(
        mut client: TcpStream,
        backends: Arc<Vec<String>>,
        counter: Arc<AtomicUsize>,
    ) {
        let index = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % backends.len();
        let backend_address = &backends[index];

        println!("Routing connection to the backend: {}", backend_address);

        let mut backend = match TcpStream::connect(backend_address).await {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!(
                    "Failed to connect to the backend {}: {}",
                    backend_address, err
                );
                return;
            }
        };

        let (client_read, mut client_write) = client.split();
        let (backend_read, mut backend_write) = backend.split();

        let client_to_backend = async {
            let mut buffer: [u8; 4096] = [0; 4096];
            loop {
                match client_read.try_read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if backend_write.write_all(&buffer[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        let backend_to_client = async {
            let mut buffer: [u8; 4096] = [0; 4096];
            loop {
                match backend_read.try_read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if client_write.write_all(&buffer[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        };

        tokio::select! {
            _ = client_to_backend => {},
            _ = backend_to_client => {}
        }
    }
}
