use crate::midi::constance::byte;
use crate::midi::util::array::u32_as_vl_vec;

pub trait IEvent {
    fn data(&self) -> Vec<byte>;
    fn occupied_bytes(&self) -> u32;
}

#[derive(Debug, Clone)]
pub enum EventType {
    NoteOn, NoteOff, ProgramChange,
    ControlChange, AfterTouch, Glide,
    KeyAfterTouch, Sysex,
}

impl EventType {
    pub fn operate_code(&self) -> byte {
        match self {
            Self::NoteOn => 0x90,
            Self::NoteOff => 0x80,
            Self::ProgramChange => 0xc0,
            Self::ControlChange => 0xb0,
            Self::AfterTouch => 0xd0,
            Self::Glide => 0xe0,
            Self::KeyAfterTouch => 0xa0,
            Self::Sysex => 0xf0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MetaEventType {
    MetaTempo, MetaEndOfTrack,
    MetaKeySignature, MetaTimeSignature,
}

impl MetaEventType {
    pub fn operate_code(&self) -> byte {
        match self {
            Self::MetaTempo => 0x51,
            Self::MetaEndOfTrack => 0x2f,
            Self::MetaKeySignature => 0x59,
            Self::MetaTimeSignature => 0x58,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub arguments: Vec<byte>,
    pub track: byte,
}

impl Event {
    pub fn new(event_type: EventType, arguments: Vec<byte>, track: byte) -> Self {
        Self {
            event_type, arguments, track
        }
    }
}

impl IEvent for Event {
    fn data(&self) -> Vec<byte> {
        let mut ret: Vec<byte> = Vec::new();
        ret.push(self.event_type.operate_code() | self.track);
        ret.append(self.arguments.clone().as_mut());
        ret
    }

    fn occupied_bytes(&self) -> u32 {
        (1 + self.arguments.len()) as u32
    }
}

#[derive(Debug, Clone)]
pub struct MetaEvent {
    pub event_type: MetaEventType,
    pub arguments: Vec<byte>
}

impl MetaEvent {
    pub fn new(event_type: MetaEventType, arguments: Vec<byte>) -> Self {
        Self {
            event_type, arguments
        }
    }

    pub fn new_default(event_type: MetaEventType) -> Self {
        Self {
            event_type, arguments: Vec::new()
        }
    }
}

impl IEvent for MetaEvent {
    fn data(&self) -> Vec<byte> {
        let mut buffer: Vec<byte> = Vec::new();
        buffer.push(self.event_type.operate_code());
        buffer.append(&mut u32_as_vl_vec(self.arguments.len() as u32));
        if self.arguments.len() != 0 {
            buffer.append(&mut self.arguments.clone());
        }
        buffer
    }

    fn occupied_bytes(&self) -> u32 {
        1 + u32_as_vl_vec(self.arguments.len() as u32).len() as u32 + self.arguments.len() as u32
    }
}

