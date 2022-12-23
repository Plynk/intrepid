
struct Node<C> where C : std::io::Read + std::io::Write 
{
    connections : Vec<C>,
    pub_msg_types : Vec<MsgTypes>,
    ip: std::net::IpAddr,   
}

pub enum MsgTypes {

    FaceId,
    LockDoor

}
