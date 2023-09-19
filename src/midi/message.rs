use crate::midi::constance::byte;
use crate::midi::event::{Event, IEvent, MetaEvent};
use crate::midi::util::array::u32_as_vl_vec;

pub trait IMessage {
    fn write_message_content(&self, buffer: &mut Vec<byte>);
    fn occupied_bytes(&self) -> u32;
    fn delta_time_array(&self) -> Vec<byte>;
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Message(Message),
    MetaMessage(MetaMessage),
    HexMessage(HexMessage),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub event: Event,
    pub time: u32
}

impl Message {
    pub fn new(event: Event, time: u32) -> Self {
        Self { event, time }
    }
}

impl IMessage for Message {

    fn write_message_content(&self, buffer: &mut Vec<byte>) {
        buffer.append(self.delta_time_array().as_mut());
        buffer.append(self.event.data().as_mut());
    }

    fn occupied_bytes(&self) -> u32 {
        self.event.occupied_bytes() + self.delta_time_array().len() as u32
    }

    fn delta_time_array(&self) -> Vec<byte> {
        u32_as_vl_vec(self.time)
    }
}

#[derive(Debug, Clone)]
pub struct HexMessage {
    hex: Vec<byte>
}

impl HexMessage {
    pub fn new(data: Vec<byte>) -> Self {
        Self { hex: data }
    }
}

impl IMessage for HexMessage {
    fn write_message_content(&self, buffer: &mut Vec<byte>) {
        buffer.append(self.hex.clone().as_mut())
    }

    fn occupied_bytes(&self) -> u32 {
        self.hex.len() as u32
    }

    fn delta_time_array(&self) -> Vec<byte> {
        panic!("delta time array is part of data in HexMessage")
    }
}

#[derive(Debug, Clone)]
pub struct MetaMessage {
    pub event: MetaEvent,
    pub time: u32,
    pub status: byte
}

impl MetaMessage {
    pub fn new(event: MetaEvent, time: u32, status: byte) -> Self {
        Self { event, time, status }
    }
}

impl IMessage for MetaMessage {
    fn write_message_content(&self, buffer: &mut Vec<byte>) {
        buffer.append(self.delta_time_array().as_mut());
        buffer.push(self.status);
        buffer.append(self.event.data().as_mut());
    }

    fn occupied_bytes(&self) -> u32 {
        1 + self.event.occupied_bytes() + self.delta_time_array().len() as u32
    }

    fn delta_time_array(&self) -> Vec<byte> {
        u32_as_vl_vec(self.time)
    }
}