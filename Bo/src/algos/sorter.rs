use crate::shared::{CMD_CHANNEL, Command};
use crate::algos::response;
use esp_println::println;
#[embassy_executor::task]
pub async fn Sorter() {
    loop {
        println!("sorting stuff");
        let Command { cat, cmd, p1, p2 } = CMD_CHANNEL.receive().await;
        match (cat, cmd) {
            (0x01, 0x01) => response::nod().await,
            _ => {}
        }
    }
}