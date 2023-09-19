use crate::midi::constance::{byte, MIDI_MAGIC_HEAD_HTRK};
use crate::midi::event::{Event, EventType, MetaEvent, MetaEventType};
use crate::midi::event::MetaEventType::MetaTempo;
use crate::midi::message::{HexMessage, IMessage, Message, MessageType, MetaMessage};
use crate::midi::util::array::{bpm2tempo_vec, u32_as_4l_byte_array, vec_put_array};
use crate::midi::util::hex::collect_message_chain_bytes;

#[derive(Debug, Clone)]
pub struct Track {
    message_chain: Vec<MessageType>,
    pub(crate) head_occupied: byte,
}
impl Track {
    pub fn new() -> Self {
        Self {
            message_chain: Vec::new(),
            head_occupied: 8
        }
    }

    pub fn seclen(&self) -> [byte; 4] {
        u32_as_4l_byte_array(collect_message_chain_bytes(&self.message_chain))
    }

    pub fn write_head(&self, buffer: &mut Vec<byte>) {
        vec_put_array(buffer, &MIDI_MAGIC_HEAD_HTRK);
        vec_put_array(buffer, &self.seclen());
    }

    pub fn message_occupied(&self) -> u32 {
        collect_message_chain_bytes(&self.message_chain)
    }

    pub fn write_message(&self, buffer: &mut Vec<byte>) {
        for message in self.message_chain.iter() {
            match message {
                MessageType::Message(message) => message.write_message_content(buffer),
                MessageType::MetaMessage(message) => message.write_message_content(buffer),
                MessageType::HexMessage(message) => message.write_message_content(buffer),
            }
        }
    }
}

#[derive(Debug)]
pub struct TrackBuilder {
    track: Track
}

impl TrackBuilder {
    pub fn new() -> Self {
        Self { track: Track::new() }
    }

    pub fn build(&self) -> Track {
        self.track.clone()
    }

    pub fn append_message(&mut self, message: Message) -> &mut Self {
        self.track.message_chain.push(MessageType::Message(message));
        self
    }

    pub fn append_meta_message(&mut self, message: MetaMessage) -> &mut Self {
        self.track.message_chain.push(MessageType::MetaMessage(message));
        self
    }

    pub fn append_hex_message(&mut self, message: HexMessage) -> &mut Self {
        self.track.message_chain.push(MessageType::HexMessage(message));
        self
    }

    pub fn meta(&mut self, meta_event: MetaEvent) -> &mut Self {
        self.append_meta_message(
            MetaMessage::new(
                meta_event,
                0,
                0xff
            )
        )
    }

    pub fn end(&mut self) -> &mut Self {
        self.meta(MetaEvent::new_default(MetaEventType::MetaEndOfTrack))
    }

    pub fn tempo(&mut self, bpm: u32) -> &mut Self {
        self.meta(MetaEvent::new(MetaTempo, bpm2tempo_vec(bpm)))
    }

    pub fn message(&mut self, event: Event, time: u32) -> &mut Self {
        self.append_message(Message::new(event, time))
    }

    pub fn message_event(&mut self, time: u32, event_type: EventType, arguments: Vec<byte>, track: byte) -> &mut Self {
        self.append_message(Message::new(Event::new(
            event_type, arguments, track
        ), time))
    }

    pub fn note_on(&mut self, note_code: byte, time: u32, velocity: byte, channel: byte) -> &mut Self {
        self.message_event(time, EventType::NoteOn, vec![note_code, velocity], channel)
    }

    pub fn note_off(&mut self, note_code: byte, time: u32, velocity: byte, channel: byte) -> &mut Self {
        self.message_event(time, EventType::NoteOff, vec![note_code, velocity], channel)
    }

}