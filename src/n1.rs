mod intrepid2;

fn main() {
    let node = intrepid2::create_node("0.0.0.0:6402".parse().expect("oh...")).unwrap();
    node.start();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
