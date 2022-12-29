use anyhow::{Ok, Result};
use std::net::UdpSocket;

const BROADCAST_SENDER: &str = "255.255.255.255";
const BROADCAST_LISTNER: &str = "0.0.0.0";

struct UDPNode {
    bind_ip: String,
    send_ip: String,
    port: String,
    tx: std::sync::mpsc::Sender<Vec<u8>>,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
}

impl UDPNode {
    pub fn new(port: String, bind_ip: String, send_ip: String) -> UDPNode {
        let (tx, rx) = std::sync::mpsc::channel();
        UDPNode {
            bind_ip,
            send_ip,
            port,
            tx,
            rx,
        }
    }
}

trait IntrepidSocket {
    fn listening_thread() -> std::sync::mpsc::Receiver<Vec<u8>>;
    fn sending_thread() -> std::sync::mpsc::Sender<Vec<u8>>;
    fn broadcast_thread() -> std::sync::mpsc::Sender<Vec<u8>>;
    fn audience_thread() -> std::sync::mpsc::Receiver<Vec<u8>>;
}
