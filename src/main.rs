mod intrepid;
use intrepid::IntrepidSocket;

fn main() {
    let socket = intrepid::UDPNode::new(
        "6402".to_string(),
        "127.0.0.1".to_string(),
        "127.0.0.2".to_string(),
    );

    let (tx, b_thread) = socket.broadcast_thread().expect("uhhhh b");
    let (rx, a_thread) = socket.audience_thread().expect("uhhhh a");

    std::thread::spawn(b_thread);
    std::thread::spawn(a_thread);

    let broadcast = move || loop {
        tx.send(vec![0, 1, 0, 1])
            .expect("send to broadcast thread failed");
        std::thread::sleep(std::time::Duration::from_secs(2))
    };

    let audience = move || loop {
        let msg = rx.recv().expect("sheesh");
        println!("{msg:?}");
    };
}
