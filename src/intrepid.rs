use anyhow::{Ok, Result};
use std::net::UdpSocket;

use uuid;

pub struct Node {
    pub uuid: String,
    pub connections: std::net::SocketAddr,
    pub pub_topics: Vec<MsgTypes>,
    pub ip: std::net::SocketAddr,
}

pub enum MsgTypes {
    FaceId,
    LockDoor,
}

impl Node {
    pub fn publish(&self) -> Result<()> {
        println!("trying to connect to socket");
        let socket = UdpSocket::bind(&self.ip.to_string())?;
        println!("connected to socket");

        let buf = vec![1, 1, 1, 1, 1, 1];

        println!("sending 1s");
        socket.send_to(&buf, self.connections.to_string())?;
        println!("sent message");

        Ok(())
    }

    pub fn recv_thread(&self) -> Result<()> {
        println!("binding socket");
        let socket = UdpSocket::bind(&self.ip.to_string())?;
        println!("starting recv");
        loop {
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)?;
            println!("{buf:?}");
        }
    }
}
