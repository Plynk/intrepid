use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{net::UdpSocket, sync::Arc};

const BROADCAST_SENDER: &str = "255.255.255.255";
const BROADCAST_LISTNER: &str = "0.0.0.0";
const BROADCAST_PORT: &str = "6401";
const BROADCAST_BUFFER_SIZE: usize = 10;

#[derive(Serialize, Deserialize)]
enum IntrepidMsg {
    Broadcast(BroadCast),
    Data(Data),
}

/////////////////////////////////////////////////
/// type     |     length     |      Data    ///

#[derive(Serialize, Deserialize)]
struct BroadCast {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct Data {
    data: Vec<u8>,
}

pub struct Intrepid {
    peers: Vec<String>,
    name: String,
}

impl Intrepid {
    pub fn new(name: String) -> Intrepid {
        Intrepid {
            peers: vec![],
            name,
        }
    }
    pub fn start<S>(&mut self, socket: S)
    where
        S: IntrepidSocket,
    {
        let (tx, b_thread) = socket.broadcast_thread().expect("uhhhh b");
        let (rx, a_thread) = socket.audience_thread().expect("uhhhh a");

        std::thread::spawn(b_thread);
        std::thread::spawn(a_thread);

        let msg = msg_to_bytes(IntrepidMsg::Broadcast(BroadCast {
            name: "blomp".to_string(),
        }));
        let broadcast = move || loop {
            tx.send(msg.clone())
                .expect("send to broadcast thread failed");
            std::thread::sleep(std::time::Duration::from_secs(2))
        };

        let audience = move || loop {
            let msg = rx.recv().expect("sheesh");
            println!("{msg:?}");
            let m = bytes_to_msg(msg);
            match m {
                IntrepidMsg::Broadcast(x) => self.peers.push(x.name),
                _ => {}
            }
        };

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

fn msg_to_bytes(s: IntrepidMsg) -> Vec<u8> {
    serde_json::to_string(&s).unwrap().into_bytes()
}

fn bytes_to_msg(b: Vec<u8>) -> IntrepidMsg {
    serde_json::from_str(std::str::from_utf8(&b[..]).unwrap()).unwrap()
}

pub struct UDPNode {
    bind_ip: String,
    send_ip: String,
    port: String,
    tx: std::sync::mpsc::Sender<Vec<u8>>,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
    broad_cast_socket: Arc<UdpSocket>,
}

impl UDPNode {
    pub fn new(port: String, bind_ip: String, send_ip: String) -> UDPNode {
        let (tx, rx) = std::sync::mpsc::channel();

        UDPNode {
            bind_ip,
            send_ip,
            port: port.clone(),
            tx,
            rx,
            broad_cast_socket: Arc::new(
                UdpSocket::bind(format!("{BROADCAST_LISTNER}:{BROADCAST_PORT}"))
                    .expect("failed to bind broadcast socket"),
            ),
        }
    }
}

impl IntrepidSocket for UDPNode {
    fn listening_thread(&self) -> std::sync::mpsc::Receiver<Vec<u8>> {
        let (tx, rx) = std::sync::mpsc::channel();
        rx
    }
    fn sending_thread(&self) -> std::sync::mpsc::Sender<Vec<u8>> {
        let (tx, rx) = std::sync::mpsc::channel();
        tx
    }
    fn broadcast_thread(
        &self,
    ) -> Result<(
        std::sync::mpsc::Sender<Vec<u8>>,
        Box<dyn Fn() -> Result<()> + Send>,
    )> {
        let socket = self.broad_cast_socket.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;
        socket.set_broadcast(true)?;
        println!("Broadcast: {:?}", socket.broadcast());
        println!("Timeout: {:?}", socket.read_timeout());

        Ok((
            tx,
            Box::new(move || {
                loop {
                    let msg = rx.recv().expect("BroadCast sender hung up");
                    socket.send_to(&msg[..], format!("{BROADCAST_SENDER}:{BROADCAST_PORT}"));
                    println!("Sent Broadcast");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
                Ok(())
            }),
        ))
    }
    fn audience_thread(
        &self,
    ) -> Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
        Box<dyn Fn() -> Result<()> + Send>,
    )> {
        let (tx, rx) = std::sync::mpsc::channel();
        let socket = self.broad_cast_socket.clone();
        Ok((
            rx,
            Box::new(move || {
                let mut buf = [0; BROADCAST_BUFFER_SIZE];
                loop {
                    let (amt, src) = socket.recv_from(&mut buf)?;
                    println!("Recieved Broadcast from : {src:?}");
                    tx.send(buf.to_vec()).expect("Audience Receiver hung up");
                }
                Ok(())
            }),
        ))
    }
}

pub trait IntrepidSocket {
    fn listening_thread(&self) -> std::sync::mpsc::Receiver<Vec<u8>>;
    fn sending_thread(&self) -> std::sync::mpsc::Sender<Vec<u8>>;
    fn broadcast_thread(
        &self,
    ) -> Result<(
        std::sync::mpsc::Sender<Vec<u8>>,
        Box<dyn Fn() -> Result<()> + Send>,
    )>;
    fn audience_thread(
        &self,
    ) -> Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
        Box<dyn Fn() -> Result<()> + Send>,
    )>;
}
