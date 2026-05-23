#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::CpuClock,
    dma::DmaDescriptor,
    i2s::master::{Config, DataFormat, I2s},
    time::Rate,
    timer::timg::TimerGroup,
};
use poi::tasks::microphone::mic_task;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// for microphone
static mut RX_DESCRIPTORS: [DmaDescriptor; 8] = [DmaDescriptor::EMPTY; 8];
const BUFFER_SIZE: usize = 1024;
static mut RX_BUFFER: [u8; BUFFER_SIZE * 4] = [0u8; BUFFER_SIZE * 4];



#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
//__________________________________________________________________________________________________

//---------------------------------------------------------------------------------------------------
// **************************      MICROPHONE INITSIALISATION *************************************
//--------------------------------------------------------------------------------------------------

    let i2s = I2s::new(
        peripherals.I2S0,
        peripherals.DMA_I2S0,
        Config::default(),
    )
    .unwrap()
    .into_async();

    let i2s_rx = i2s.i2s_rx
        .with_bclk(peripherals.GPIO33)
        .with_ws(peripherals.GPIO25)
        .with_din(peripherals.GPIO32)
        .build(unsafe { &mut *(&raw mut RX_DESCRIPTORS) });

    spawner.spawn(mic_task(i2s_rx).expect("microphone failed"));
//_________________________________________________________________________________________________
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}