use crate::midi::message::{IMessage, MessageType};

pub fn bpm2tempo(bpm: u32) -> u32 { (60 * 1000000) / bpm }

pub fn collect_message_chain_bytes(message_chain: &Vec<MessageType>) -> u32 {
    let sum: u32 = message_chain
        .iter()
        .map(| item| match item {
            MessageType::Message(message) => message.occupied_bytes(),
            MessageType::MetaMessage(message) => message.occupied_bytes(),
            MessageType::HexMessage(message) => message.occupied_bytes(),
        })
        .sum();
    sum
}