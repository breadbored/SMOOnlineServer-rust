use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum PacketType {
    Unknown,
    Init,
    Player,
    Cap,
    Game,
    Tag,
    Connect,
    Disconnect,
    Costume,
    Shine,
    Capture,
    ChangeStage,
    Command
}

impl TryFrom<u16> for PacketType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == PacketType::Unknown as u16 => Ok(PacketType::Unknown),
            x if x == PacketType::Init as u16 => Ok(PacketType::Init),
            x if x == PacketType::Player as u16 => Ok(PacketType::Player),
            x if x == PacketType::Cap as u16 => Ok(PacketType::Cap),
            x if x == PacketType::Game as u16 => Ok(PacketType::Game),
            x if x == PacketType::Tag as u16 => Ok(PacketType::Tag),
            x if x == PacketType::Connect as u16 => Ok(PacketType::Connect),
            x if x == PacketType::Disconnect as u16 => Ok(PacketType::Disconnect),
            x if x == PacketType::Costume as u16 => Ok(PacketType::Costume),
            x if x == PacketType::Shine as u16 => Ok(PacketType::Shine),
            x if x == PacketType::Capture as u16 => Ok(PacketType::Capture),
            x if x == PacketType::ChangeStage as u16 => Ok(PacketType::ChangeStage),
            x if x == PacketType::Command as u16 => Ok(PacketType::Command),
            _ => Ok(PacketType::Unknown),
        }
    }
}

impl TryFrom<PacketType> for u16 {
    type Error = ();

    fn try_from(v: PacketType) -> Result<Self, Self::Error> {
        Ok(v as u16)
    }
}