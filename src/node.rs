mod intrepid;
use intrepid::IntrepidSocket;

fn main() {
    let socket = intrepid::UDPNode::new("6403".into(), "127.0.0.1".into(), "127.0.0.1".into());
    let (tx, b_thread) = socket.broadcast_thread().expect("uhhhh b");
    let (rx, a_thread) = socket.audience_thread().expect("uhhhh a");

    std::thread::spawn(b_thread);
    std::thread::spawn(a_thread);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        tx.send(vec![1,1,1,1]);
    }
}
