use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{net::UdpSocket, sync::Arc};

const BROADCAST_SENDER: &str = "255.255.255.255";
const BROADCAST_LISTNER: &str = "0.0.0.0";
const BROADCAST_PORT: &str = "6401";
const BROADCAST_BUFFER_SIZE: usize = 10;

#[binrw]
#[brw(magic = b"\xFE\xEF", little)]
struct IntrepidMsgFrame {
    mtype: IntrepidMsgType,
    length: u32,
    #[br(count = length)]
    data: Vec<u8>,
    chk: u16,
}

#[binrw]
enum IntrepidMsgType {
    #[brw(magic = b"\x00\x00")]
    Broadcast,
    Data,
}

enum IntrepidMsg {
    BroadCast(BroadCast),
    Data(Data),
}
impl IntrepidMsg {
    fn encode(&self, buf: &mut Vec<u8>) -> anyhow::Result<()> {
        let frame = IntrepidMsgFrame{
            mtype : self.into_msg_type(),
            

        }
        anyhow::Ok(())
    }

    fn into_msg_type(&self) -> IntrepidMsgType {
        match *self {
            IntrepidMsg::BroadCast(_) => IntrepidMsgType::Broadcast,
            IntrepidMsg::Data(_) => IntrepidMsgType::Data,
        }
    }
}
/////////////////////////////////////////////////
/// type     |     length     |      Data    ///

struct BroadCast {
    name: String,
}
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

        // let (b_tx, b_rx) = std::sync::mpsc::channel();

        let audience = move || loop {
            let msg = rx.recv().expect("sheesh");
            println!("{msg:?}");
            // let m = bytes_to_msg(msg);
            // match m {
            //     IntrepidMsg::Broadcast(x) => b_tx.send(x.name).expect("b_tx hung up"),
            //     _ => {}
            // }
        };
        std::thread::spawn(broadcast);
        std::thread::spawn(audience);

        loop {
            // let m = b_rx.recv();
            println!("Receiving");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
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
    fn audience_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
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
                    tx.send(buf.to_vec()).expect("Audience Receiver hung up");
                }
                anyhow::Ok(())
            }),
        ))
    }
}

pub trait IntrepidSocket {
    fn listening_thread(&self) -> std::sync::mpsc::Receiver<Vec<u8>>;
    fn sending_thread(&self) -> std::sync::mpsc::Sender<Vec<u8>>;
    fn broadcast_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Sender<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
    fn audience_thread(
        &self,
    ) -> anyhow::Result<(
        std::sync::mpsc::Receiver<Vec<u8>>,
        Box<dyn Fn() -> anyhow::Result<()> + Send>,
    )>;
}
