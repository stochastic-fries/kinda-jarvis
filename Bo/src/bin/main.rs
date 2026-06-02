#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_hal::i2c::master::{I2c, Config as I2cConfig};

use Bo::tasks::wifi::{init_wifi, net_task, wifi_connection_task, wifi};
use Bo::tasks::servo::ServoController;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]


#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o esp -o alloc -o unstable-hal -o wifi -o embassy

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);
//__________________________________________________________________________________________________________________
//                                  
//                                              --WIFI--

    //let (mut _wifi_controller, _interfaces) =
    //    esp_radio::wifi::new(peripherals.WIFI, Default::default())
    //        .expect("Failed to initialize Wi-Fi controller");

    let (wifi_controller, runner, stack) = init_wifi(peripherals.WIFI);
    spawner.spawn(wifi_connection_task(wifi_controller).expect("problem with wifi connector"));
    spawner.spawn(net_task(runner).expect("problem with wifi daemon"));
    spawner.spawn(wifi(stack).expect("there's a problem in the wifi programe"));
//___________________________________________________________________________________________________________________

//
//                                              --servos--
//                                          (via PCA9685 (i2c))

    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
    .unwrap()
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22)
    .into_async();

    let mut servos = ServoController::new(i2c).await;
//___________________________________________________________________________________________________
loop {
        Timer::after(Duration::from_secs(1)).await;
    }

}
