use binrw::{binrw, BinRead, BinWrite};


#[derive(Debug)]
#[binrw]
#[brw(magic = b"\xFE\xEF", little)]
pub struct IntrepidMsgFrame {
    mtype: IntrepidMsgType,
    length: u32,
    #[br(count = length)]
    data: Vec<u8>,
}

#[binrw]
#[derive(Debug, Clone)]
pub enum IntrepidMsgType {
    #[brw(magic = b"\x00\x00")]
    Broadcast,
    #[brw(magic = b"\x00\x01")]
    Data,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct BroadCast {
    pub id: u32,
}
#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Data {
    length: u32,
    #[br(count = length)]
    d: Vec<u8>,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub enum IntrepidMsg {
    BroadCast(BroadCast),
    Data(Data),
}

impl IntrepidMsg {
    pub fn into_frame(&self) -> IntrepidMsgFrame {
        let mut writer = std::io::Cursor::new(vec![]);
        self.write(&mut writer).expect("failed to write");
        let length = writer.get_ref().len() as u32;

        IntrepidMsgFrame {
            mtype: self.into_msg_type(),
            length,
            data: writer.into_inner(),
        }
    }

    fn into_msg_type(&self) -> IntrepidMsgType {
        match *self {
            IntrepidMsg::BroadCast(_) => IntrepidMsgType::Broadcast,
            IntrepidMsg::Data(_) => IntrepidMsgType::Data,
        }
    }
}

impl IntrepidMsgFrame {
    pub fn into_msg(&mut self) -> IntrepidMsg {
        let mut buf = std::io::Cursor::new(self.data.clone());
        match self.mtype {
            IntrepidMsgType::Data => {
                IntrepidMsg::Data(Data::read(&mut buf).expect("failed to read into data"))
            }
            IntrepidMsgType::Broadcast => IntrepidMsg::BroadCast(
                BroadCast::read(&mut buf).expect("failed to read into broadcast"),
            ),
        }
    }
}
