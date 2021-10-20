use std::thread;
use zmq::Socket;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct BotService {
    context: zmq::Context,
    pub zmq_listen_addr: SocketAddr,
    publisher: Option<Socket>,
}

impl BotService {
    pub async fn new() -> Self {
        Self {
            context: zmq::Context::new(),
            zmq_listen_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 28332),
            publisher: None,
        }
    }

    pub fn start_zmq_service(
        mut self,
    ) -> Result<(), String> {
        println!("Running ZMQ Server");

        self.publisher = Some(self.context.socket(zmq::PUB).unwrap());
        self.publisher.as_ref().unwrap().bind(self.zmq_listen_addr.to_string().as_str()).expect("Unable to bind publisher for ZMQ server");

        loop {
            self.publisher
                .unwrap()
                .send("notuserjoin", zmq::SNDMORE)
                .expect("failed sending first envelope");
            self.publisher
                .unwrap()
                .send("We don't want to see this", 0)
                .expect("failed sending first message");
            self.publisher
                .unwrap()
                .send("userjoin", zmq::SNDMORE)
                .expect("failed sending second envelope");
            self.publisher
                .unwrap()
                .send("We would like to see this", 0)
                .expect("failed sending second message");
            thread::sleep(Duration::from_millis(1));
        }


        Ok(())
    }

}

pub fn run_zmq_server(ctx: &mut zmq::Context, size: u64, workers: u64) {
}
