use esp_hal::{i2s::master::I2sRx, Async};
use esp_println::println;

const BUFFER_SIZE: usize = 1024;

#[embassy_executor::task]
pub async fn mic_task(mut i2s_rx: I2sRx<'static, Async>) {
    let mut buffer = [0u8; BUFFER_SIZE * 4];
    let mut count = 0u32;

    loop {
        match i2s_rx.read_dma_async(&mut buffer).await {
            Ok(_) => {
    let sample = i16::from_le_bytes([buffer[0], buffer[1]]);

    if count % 100 == 0 {
        println!("frame {}: sample = {}", count, sample);
    }
    count += 1;
}
            Err(e) => {
                println!("I2S read error: {:?}", e);
            }
        }
    }
}