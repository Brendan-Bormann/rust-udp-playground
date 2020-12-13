use serde::{Deserialize, Serialize};
use rmp_serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub sender: String,
    pub packet_type: String,
    pub packet_data: String
}

impl Packet {
    pub fn serialize_packet(packet: &Packet) -> Vec<u8> {
        rmp_serde::to_vec(packet).expect("Failed to serrialize packet.")
    }

    pub fn deserialize_packet(data: &Vec<u8>) -> Packet {
        rmp_serde::from_read_ref(data).expect("Failed to serrialize packet.")
    }
}