use framing::{FrameType, Frame, MethodFrame};
use amqp_error::AMQPResult;

pub trait Method {
    fn decode(method_frame: MethodFrame) -> AMQPResult<Self> where Self: Sized;
    fn encode(&self) -> AMQPResult<Vec<u8>>;
    fn name(&self) -> &'static str;
    fn id(&self) -> u16;
    fn class_id(&self) -> u16;

    fn encode_method_frame(&self) -> AMQPResult<Vec<u8>> {
        let frame = MethodFrame { class_id: self.class_id(), method_id: self.id(), arguments: try!(self.encode()) };
        frame.encode()
    }

    fn to_frame(&self, channel: u16) -> AMQPResult<Frame> {
        Ok(Frame {
            frame_type: FrameType::METHOD,
            channel: channel,
            payload: try!(self.encode_method_frame()),
        })
    }
}
