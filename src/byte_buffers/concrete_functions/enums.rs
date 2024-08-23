
#[repr(u32)]
pub enum MessageCode {
    ProducerMsg = 1000,
}

pub enum ProducerErrorCode{
    NoTopicExists = 2000,
}

pub enum ExceptionCode{
    Ignore = 500,
    DisconnectClient = 501
}