use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub packet_id: String,
    pub version: u8,
    pub cmd_type: u8,
    pub timestamp: SystemTime,
    pub payload: Vec<String>
}

pub fn decode(pkt: Vec<u8>) -> Packet {
    deserialize(&pkt[..]).unwrap()
}

pub fn encode(pkt: Packet) -> Vec<u8> {
    serialize(&pkt).unwrap()
}