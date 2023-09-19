
use tokio::sync::broadcast;
use std::{path::PathBuf};
use async_trait::async_trait;

use super::super::utils::validation;

#[async_trait]
pub trait ServerTrait {
    fn new() -> Self;

     async fn runner(&self);
}

#[derive(Default, Clone, Debug)]
pub struct Message {
    pub connect: bool,
    pub terminate: bool,
    pub path: PathBuf,
    pub bind_address: String,
    pub port: u16,
}

pub struct Server { 
    pub sender: broadcast::Sender<Message>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            sender: broadcast::channel(10).0,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server::default()
    }

    pub fn start(&self, path: PathBuf, bind_address: String, port: u16) {

        validation::validate_ip_port(&format!("{}:{}", bind_address, port)).expect("Invalid IP");
        validation::validate_path(&path).expect("Invalid path");

        let s = Message{connect: true, terminate: false, path, bind_address, port};
        self.sender.send(s);
    }

    pub fn stop(&self){
        let mut m = Message::default();
        m.connect = false;
        m.terminate = false;
        self.sender.send(m);
    }

    pub fn terminate(&self){
        // First stop and to then terminate
        let mut m = Message::default();
        m.connect = false;
        m.terminate = true;
        // Send twice. Once to make sure the server is terminated (inner loop)
        // and the second to ensure runner exits.
        let _r1 = self.sender.send(m.clone());
        let _r2 = self.sender.send(m);
    }
}
