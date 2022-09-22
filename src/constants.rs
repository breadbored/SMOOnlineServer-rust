use crate::packet::{
    PacketType::PacketType,
};

const COSTUME_NAME_SIZE: usize = 0x20;

pub fn packet_to_type_map(key: &str) -> PacketType {
    match key {
        "CapPacket" => PacketType::Cap,
        "CapturePacket" => PacketType::Capture,
        "ChangeStagePacket" => PacketType::ChangeStage,
        "ConnectPacket" => PacketType::Connect,
        "CostumePacket" => PacketType::Costume,
        "DisconnectPacket" => PacketType::Disconnect,
        "GamePacket" => PacketType::Game,
        "InitPacket" => PacketType::Init,
        "PlayerPacket" => PacketType::Player,
        "ShinePacket" => PacketType::Shine,
        "TagPacket" => PacketType::Tag,
        "UnhandledPacket" => PacketType::Unknown,
        _ => PacketType::Unknown,
    }
}

pub fn type_to_packet_map(key: PacketType) -> &'static str {
    match key {
        PacketType::Cap => "CapPacket",
        PacketType::Capture => "CapturePacket",
        PacketType::ChangeStage => "ChangeStagePacket",
        PacketType::Connect => "ConnectPacket",
        PacketType::Costume => "CostumePacket",
        PacketType::Disconnect => "DisconnectPacket",
        PacketType::Game => "GamePacket",
        PacketType::Init => "InitPacket",
        PacketType::Player => "PlayerPacket",
        PacketType::Shine => "ShinePacket",
        PacketType::Tag => "TagPacket",
        PacketType::Unknown => "UnhandledPacket",
        PacketType::Command => "UnhandledPacket", // TODO: What?
    }
}