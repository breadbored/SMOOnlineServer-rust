use uuid::Uuid;

pub const MAX_PLAYERS: u16 = 8;

pub struct Settings {
    pub server: ServerTable,
    pub scenario: ScenarioTable,
    pub banned_players: BannedPlayers,
    pub flip: FlipTable,
    pub discord: DiscordTable,
    pub shine: ShineTable,
    pub persist_shines: PersistShinesTable,
}

impl Settings {
    pub fn defaults() -> Self {
        Settings {
            server: ServerTable {
                address: "0.0.0.0".to_string(),
                port: 1027,
                max_players: 0
            },
            scenario: ScenarioTable {
                merge_enabled: false
            },
            banned_players: BannedPlayers {
                enabled: false,
                players: Vec::new(),
                ip_addresses: Vec::new()
            },
            flip: FlipTable {
                enabled: false,
                players: Vec::new(),
                pov: FlipOptions::BothOption
            },
            discord: DiscordTable {
                token: None,
                prefix: "$".to_string(),
                command_channel: None,
                log_channel: None,
            },
            shine: ShineTable {
                enabled: true
            },
            persist_shines: PersistShinesTable {
                enabled: true,
                file_name: "./moons.json".to_string()
            }
        }
    }
}

pub enum FlipOptions {
    BothOption,
    SelfOption,
    OthersOption
}

pub struct ServerTable {
    pub address: String,
    pub port: u16,
    pub max_players: u16,
}

pub struct ScenarioTable {
    pub merge_enabled: bool,
}

pub struct BannedPlayers {
    pub enabled: bool,
    pub players: Vec<Uuid>,
    pub ip_addresses: Vec<String>,
}

pub struct FlipTable {
    pub enabled: bool,
    pub players: Vec<Uuid>,
    pub pov: FlipOptions,
}

pub struct DiscordTable {
    pub token: Option<String>,
    pub prefix: String,
    pub command_channel: Option<String>,
    pub log_channel: Option<String>,
}

pub struct ShineTable {
    pub enabled: bool,
}

pub struct PersistShinesTable
{
    pub enabled: bool,
    pub file_name: String,
}