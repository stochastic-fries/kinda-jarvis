#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_net::{Config as NetConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};

use esp_hal::{
    analog::dac::Dac,
    clock::CpuClock,
    dma::DmaDescriptor,
    i2s::master::{Config, I2s},
    timer::timg::TimerGroup,
};

use crate::alloc::string::ToString;
use esp_println::println;
use esp_radio::wifi::{
    Config as WifiConfig, ControllerConfig, Interface, WifiController,
    sta::StationConfig,
};

use static_cell::StaticCell;
use poi::tasks::microphone::mic_task;
use poi::tasks::wifi::wifi_task;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const SSID:     &str = "";
const PASSWORD: &str = "";

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

static mut RX_DESCRIPTORS: [DmaDescriptor; 8] = [DmaDescriptor::EMPTY; 8];

#[embassy_executor::task]
async fn wifi_connection_task(mut controller: WifiController<'static>) {
    println!("connecting to wifi...");
    controller.connect_async().await.unwrap();
    println!("wifi connected!");

    loop {
        Timer::after(Duration::from_secs(5)).await;
        controller.connect_async().await.ok();
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}

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

//________________________________________________________________________________________________

// ------------------------------WIFI-------------------------------------------------

    let station_config = StationConfig::default()
        .with_ssid(SSID)
        .with_password(PASSWORD.to_string());
    let controller_config = ControllerConfig::default()
        .with_initial_config(WifiConfig::Station(station_config));

    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, controller_config).unwrap();

    let resources = STACK_RESOURCES.init(StackResources::new());

    let (stack, runner) = embassy_net::new(
        interfaces.station,
        NetConfig::dhcpv4(Default::default()),
        resources,
        12345u64,
    );

    spawner.spawn(wifi_connection_task(wifi_controller).expect("wifi conn failed"));
    spawner.spawn(net_task(runner).expect("net task failed"));
    spawner.spawn(wifi_task(stack).expect("wifi task failed"));
//___________________________________________________________________________________________________

//----------------------------------MIC---------------------------------------------------

    let i2s = I2s::new(
        peripherals.I2S0,
        peripherals.DMA_I2S0,
        Config::default(),
    )
    .unwrap()
    .into_async();

    let i2s_rx = i2s.i2s_rx
        .with_bclk(peripherals.GPIO33)
        .with_ws(peripherals.GPIO26)
        .with_din(peripherals.GPIO32)
        .build(unsafe { &mut *(&raw mut RX_DESCRIPTORS) });

    spawner.spawn(mic_task(i2s_rx).expect("microphone failed"));
//____________________________________________________________________________________________


    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}