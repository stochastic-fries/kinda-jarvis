// path way for all the stuff which'll be sent fromm file to file
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

pub struct Command {
    pub cat: u8,
    pub cmd: u8,
    pub p1:  u8,
    pub p2:  u8,
}

pub static CMD_CHANNEL: Channel<CriticalSectionRawMutex, Command, 8> = Channel::new();

