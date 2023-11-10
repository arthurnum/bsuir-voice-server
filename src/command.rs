use serde::{Serialize, Deserialize};

pub const CMD_INDEX: u8 = 1;
pub const CMD_POST_RECORD: u8 = 100;
pub const CMD_GET_RECORD: u8 = 120;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: u8,
    pub data_len: u64,
    pub timestamp: u64,
}

pub const COMMAND_BIN_SIZE: usize = 1 + 8 + 8;
