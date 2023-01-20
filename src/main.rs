mod intrepid;
use intrepid::IntrepidSocket;

fn main() {
    let m = intrepid::IntrepidMsg::BroadCast(intrepid::BroadCast { id: 7 });
    println!("intrepidMSG : {m:?}");
    let mut d = m.into_frame();
    println!("frame : {m:?}");
    let m2 = d.into_msg();
    println!("msg2 : {m2:?}");
}
