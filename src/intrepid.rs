use crate::protocal::*;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{net::UdpSocket, sync::Arc};

const BROADCAST_SENDER: &str = "255.255.255.255";
const BROADCAST_LISTNER: &str = "0.0.0.0";
const BROADCAST_PORT: &str = "6401";
const BROADCAST_BUFFER_SIZE: usize = 100;

// length until peer is set to inactive
const ACTIVE_LENGTH: std::time::Duration = std::time::Duration::from_secs(5);

#[derive(Debug)]
struct IntrepidPeer {
    ip: std::net::SocketAddr,
    last_seen: std::time::SystemTime,
    name: u32,
    active: bool,
}

#[derive(Debug)]
pub struct Intrepid {
    peers: Vec<IntrepidPeer>,
    name: u32,
}

impl Intrepid {
    pub fn new(name: u32) -> Intrepid {
        Intrepid {
            peers: vec![],
            name,
        }
    }

    // THIS FUNCTION MODIFIES THE STATE OF "PEERS" UNDETERMINISTALLY
    fn check_peer_heartbeat(&mut self) {
        self.peers
            .iter_mut()
            .filter(|p| p.last_seen.elapsed().expect("failed to get time elapsed") > ACTIVE_LENGTH)
            .for_each(|p| p.active = false);
    }

    fn add_or_update_peer(&mut self, peer: IntrepidPeer) -> anyhow::Result<()> {
        let found = self.peers.iter_mut().fold(false, |f, mut x| {
            if x.name == peer.name {
                x.last_seen = peer.last_seen;
                x.active = true;
                return true || f;
            } else {
                return false || f;
            }
        });

        if !found {
            self.peers.push(peer);
        }

        anyhow::Ok(())
    }

    pub fn start<S>(&mut self, socket: S)
    where
        S: IntrepidSocket,
    {
        let (brodacast_tx, b_thread) = socket.broadcast_thread().expect("uhhhh b");
        let (audience_rx, a_thread) = socket.audience_thread().expect("uhhhh a");
        let (sender_tx, s_thread) = socket.sending_thread().expect("uhhhh s");
        let (listener_rx, l_thread) = socket.listening_thread().expect("uhhh l");

        std::thread::spawn(b_thread);
        std::thread::spawn(a_thread);
        std::thread::spawn(l_thread);
        std::thread::spawn(s_thread);

        let mut b = std::io::Cursor::new(vec![]);
        IntrepidMsg::BroadCast(BroadCast { name: self.name })
            .into_frame()
            .write(&mut b)
            .expect("failed write broadcast msg into bytes");

        let broadcast = move || loop {
            // println!("broadcast send {b:?}");
            brodacast_tx
                .send(b.clone().into_inner())
                .expect("send to broadcast thread failed");
            std::thread::sleep(std::time::Duration::from_secs(2))
        };

        let mut d = std::io::Cursor::new(vec![]);
        IntrepidMsg::Data(Data {
            length: 1,
            d: vec![10],
        }).into_frame().write(&mut d).expect("failed to write data msg into bytes");
        let sender = move || loop {
            sender_tx
                .send((
                    "127.0.0.1:6401".parse().expect("fuck ip"),
                    d.clone().into_inner(),
                ))
                .expect("send to sender thread failed");
            std::thread::sleep(std::time::Duration::from_secs(2));
            // println!("sending {d:?}");
        };

        let (p_tx, p_rx) = std::sync::mpsc::channel();
        let pp_tx = p_tx.clone();

        let audience = move || loop {
            let (mut msg, src) = audience_rx.recv().expect("sheesh");
            let mut msg = std::io::Cursor::new(msg);
            // println!("audience recv: {msg:?}");
            match IntrepidMsgFrame::read(&mut msg) {
                Ok(mut m) => {
                    let msg = m.into_msg();
                    pp_tx.send((msg, src));
                    // println!("{msg:?}");
                }
                Err(_) => println!("failed to read broadcast"),
            }
        };

        let listner = move || loop {
            let mut msg = listener_rx
                .recv()
                .expect("listner thread : channel disconnected");
            let mut msg = std::io::Cursor::new(msg);
            match IntrepidMsgFrame::read(&mut msg) {
                Ok(mut m) => {
                    let msg = m.into_msg();
                    p_tx.send((msg, "127.0.0.1".parse().expect("shitttt")));
                }
                Err(_) => println!("failed to read data"),
            }
        };

        std::thread::spawn(broadcast);
        std::thread::spawn(audience);
        std::thread::spawn(sender);
        std::thread::spawn(listner);

        loop {
            // TEMP HEART BEAT CHECKING NEEDS TO BE DONE ELSEWHERE
            self.check_peer_heartbeat();

            let (msg, src) = p_rx.recv().expect("main thread senders hung up");
            match msg {
                IntrepidMsg::BroadCast(x) => {
                    let peer = broadcast_to_peer(x, src);
                    self.add_or_update_peer(peer);
                }
                IntrepidMsg::Data(x) => {
                    println!("{x:?}")
                }
            }
            println!("{self:?}");

            // std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

pub struct UDPNode {
    port: String,
    bind_socket: Arc<UdpSocket>,
    broad_cast_socket: Arc<UdpSocket>,
}

impl UDPNode {
    pub fn new(port: String, bind_ip: String) -> UDPNode {
        UDPNode {
            port: port.clone(),
            bind_socket: Arc::new(
                UdpSocket::bind(format!("{bind_ip}:{port}"))
                    .expect("failed to bind to bind socket"),
            ),
            broad_cast_socket: Arc::new(
                UdpSocket::bind(format!("{BROADCAST_LISTNER}:{BROADCAST_PORT}"))
                    .expect("failed to bind broadcast socket"),
            ),
        }
    }
}

impl IntrepidSocket for UDPNode {
    // returns a thread that transmitst received bytes
    fn listening_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )> {
        let (tx, rx) = std::sync::mpsc::channel();
        let socket = self.bind_socket.clone();
        Ok((
            rx,
            Box::new(move || {
                let mut buf = [0; BROADCAST_BUFFER_SIZE];

                loop {
                    let (src, msg) = socket
                        .recv_from(&mut buf)
                        .expect("failed to read bind socket");
                    tx.send(buf.to_vec())
                        .expect("failed to send lister msg to main thread");
                }

                anyhow::Ok(())
            }),
        ))
    }

    // returns a thread that listenst to the channel which will supply a destination and bytes
    fn sending_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Sender<(std::net::SocketAddr, Vec<u8>)>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )> {
        let (tx, rx) = std::sync::mpsc::channel();
        let socket = self.bind_socket.clone();

        anyhow::Ok((
            tx,
            Box::new(move || {
                loop {
                    let (des, msg) = rx.recv().expect("sending thread : channel hung up");
                    socket
                        .send_to(&msg[..], des)
                        .expect("sending thread : socket failed to send");
                }

                anyhow::Ok(())
            }),
        ))
    }

    // returns a thread that will send data to all nodes on network through a channel
    fn broadcast_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Sender<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
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
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
                Ok(())
            }),
        ))
    }
    // listens for broadcasts and returns the message being broadcasted
    fn audience_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<(Vec<u8>, std::net::SocketAddr)>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )> {
        let (tx, rx) = std::sync::mpsc::channel();
        let socket = self.broad_cast_socket.clone();
        Ok((
            rx,
            Box::new(move || {
                let mut buf = [0; BROADCAST_BUFFER_SIZE];
                loop {
                    let (amt, src) = socket.recv_from(&mut buf)?;
                    tx.send((buf.to_vec(), src))
                        .expect("Audience Receiver hung up");
                }
                anyhow::Ok(())
            }),
        ))
    }
}

pub trait IntrepidSocket {
    fn listening_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
    fn sending_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Sender<(std::net::SocketAddr, Vec<u8>)>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
    fn broadcast_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Sender<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
    fn audience_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<(Vec<u8>, std::net::SocketAddr)>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
}

fn broadcast_to_peer(b_msg: BroadCast, ip: std::net::SocketAddr) -> IntrepidPeer {
    IntrepidPeer {
        ip,
        last_seen: std::time::SystemTime::now(),
        name: b_msg.name,
        active: true,
    }
}
