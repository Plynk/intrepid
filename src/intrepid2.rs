use anyhow::{Ok, Result};
use std::net::UdpSocket;
use std::sync::Arc;

use uuid;

#[derive(Clone)]
pub struct Node {
    uuid: String,
    connections: Vec<std::net::SocketAddr>,
    pub_topics: Vec<MsgTypes>,
    ip: std::net::SocketAddr,
    socket: Arc<UdpSocket>,
}

pub fn create_node(ip: std::net::SocketAddr) -> Result<Node> {
    let socket = Arc::new(UdpSocket::bind(ip)?);
    Ok(Node {
        uuid: ip.to_string(),
        connections: vec![],
        pub_topics: vec![],
        ip,
        socket,
    })
}

#[derive(Clone)]
pub enum MsgTypes {
    FaceId,
    LockDoor,
}

impl Node {
    /////////TEST/////////////////////////////////////////////
    // pub fn publish(&self) -> Result<()> {
    //     println!("trying to connect to socket");
    //     let socket = UdpSocket::bind(&self.ip.to_string())?;
    //     println!("connected to socket");

    //     let buf = vec![1, 1, 1, 1, 1, 1];

    //     println!("sending 1s");
    //     socket.send_to(&buf, self.connections.to_string())?;
    //     println!("sent message");

    //     Ok(())
    // }

    // pub fn recv_thread(&self) -> Result<()> {
    //     println!("binding socket");
    //     let socket = UdpSocket::bind(&self.ip.to_string())?;
    //     println!("starting recv");
    //     loop {
    //         let mut buf = [0; 10];
    //         let (amt, src) = socket.recv_from(&mut buf)?;
    //         println!("{buf:?}");
    //     }
    // }
    //////////////////////////////////////////////////////////
    pub fn start(&self) -> Result<()> {
        std::thread::spawn(broadcast_thread().unwrap());
        std::thread::spawn(reciever_thread(self.socket.clone()).unwrap());

        Ok(())
    }
}

// listens for broadcasts of new nodes joining the netwrok
// updates network
// listens for multicasts of relevant topics
fn reciever_thread(socket: Arc<UdpSocket>) -> Result<impl Fn() -> Result<()>> {
    Ok(move || loop {
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("src : {src:?} \n data : {buf:?}");
    })
}

// fn publish_thread<F>(socket: Arc<UdpSocket>) -> Result<(F)>
// where
//     F: FnOnce() -> Result<()>,
// {
//     Ok()
// }

// broadcasts this node at 1hz
fn broadcast_thread() -> Result<impl Fn() -> Result<()>> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:6401")?;
    socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;
    socket.set_broadcast(true)?;
    println!("Broadcast: {:?}", socket.broadcast());
    println!("Timeout: {:?}", socket.read_timeout());

    Ok(move || {
        loop {
            socket.send_to(&[1, 1, 1, 0, 0], "255.255.255.255:6401");
            println!("Sent Broadcast");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        Ok(())
    })
}
