
mod intrepid;

fn main(){
   let node =  intrepid::Node{
       uuid: "n2".to_string(),
        ip : "127.0.0.1:6401".parse().expect("uhh"),
        connections : "127.0.0.1:6402".parse().expect("uhh"),
        pub_topics : vec![intrepid::MsgTypes::FaceId] 
    };

    node.recv_thread();


}
