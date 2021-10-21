use std::thread;
use zmq::Socket;
use log::{info, warn};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct BotService {
    context: zmq::Context,
    zmq_listen_addr: SocketAddr,
    publisher: Option<Socket>,
    pub user_join_channel: Sender<String>,
    user_join_channel_receiver: Receiver<String>,
}

impl BotService {
    pub fn new() -> Self {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        Self {
            context: zmq::Context::new(),
            zmq_listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 28332),
            publisher: None,
            user_join_channel: tx,
            user_join_channel_receiver: rx,
        }
    }

    pub fn listen_address(mut self, address: SocketAddr) -> Self {
        self.zmq_listen_addr = address;
        self
    }

    pub fn start_zmq_service(
        mut self,
    ) -> Self {

        self.publisher = Some(self.context.socket(zmq::PUB).unwrap());
        self.publisher.as_ref().unwrap().bind(format!("tcp://{}", self.zmq_listen_addr).as_str()).expect("Unable to bind publisher for ZMQ server");
        info!("Running ZMQ Server on tcp://{}", self.zmq_listen_addr);
        tokio::spawn( async move {
            loop {
                match &self.user_join_channel_receiver.recv() {
                    Ok(id) => {
                        self.publisher
                            .as_ref()
                            .unwrap()
                            .send("userjoin", zmq::SNDMORE)
                            .expect("failed sending user join envelope");
                        self.publisher
                            .as_ref()
                            .unwrap()
                            .send(&id.as_str(), 0)
                            .expect("failed sending user join message");

                    },
                    Err(e) => {
                        warn!("Could not process new user session id {}", e);
                        continue;
                    }
                };
            }
        });
        self
    }
}
