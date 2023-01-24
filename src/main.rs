mod intrepid;
mod protocal;
use intrepid::IntrepidSocket;

use binrw::{binrw, BinRead, BinWrite};

fn main() {
    // let mut s = intrepid::UDPNode::new("6405".into(), "10.0.0.9".into(), "10.0.0.9".into());
    // let mut n = intrepid::Intrepid::new(0);

    // n.start(s);

    // loop {
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    //     println!(".");
    // }
    //

    let m = protocal::IntrepidMsg::BroadCast(protocal::BroadCast { id: 32 });
    println!("msg : {m:?}");
    let m = m.into_frame();
    println!("msg: {m:?}");

    let mut b = std::io::Cursor::new(vec![]);
    m.write(&mut b).expect("uh oh1");
    println!("bytes : {b:?}");
    b.set_position(0);
    let mut m = protocal::IntrepidMsgFrame::read(&mut b).expect("uh oh");
    println!("frame : {m:?}");
    let m = m.into_msg();
    println!("msg : {m:?}");
}
