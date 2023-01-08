mod intrepid;
use intrepid::IntrepidSocket;

fn main() {
    let mut socket = intrepid::UDPNode::new(
        "6402".to_string(),
        "127.0.0.1".to_string(),
        "127.0.0.2".to_string(),
    );

    let mut mesh = intrepid::Intrepid::new("king".to_string());

    mesh.start(socket);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
