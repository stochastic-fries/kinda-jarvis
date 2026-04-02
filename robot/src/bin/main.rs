#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
//  default/req.
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;
use log::info;
// stuff import
use esp_hal::i2c::master::{I2c, Config as I2cConfig};
use pwm_pca9685::{Pca9685, Address , Channel};
use esp_hal::Async;
use esp_hal::gpio::{Input, Pull, InputConfig};   //btn 

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
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
    let _connector = BleConnector::new(&radio_init, peripherals.BT, Default::default());

    //==================================================================
    //                  I2c setup
    //====================================================================== 
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default()).unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22)
        .into_async();

    //=====================================================================
    //                  servo driver (PCA9685)
    //=====================================================================
    let mut pca = Pca9685::new(i2c, Address::default()).unwrap();
    pca.set_prescale(121).await.unwrap();  // 50Hz
    pca.enable().await.unwrap();

    fn angle_to_count(deg: f32) -> u16 {
           let pulse_ms = 1.0_f32 + (deg.clamp(0.0, 180.0) / 180.0) * 1.0;
           (pulse_ms / 20.0 * 4096.0) as u16
    }
    let mut current : u16= 260; //center
    Timer::after(Duration::from_secs(5)).await;
    //----------------------------------------------------------------------
    //              li'l guide
    // servo hardware min is at 90 and max is at 480 
    //
    //-----------------------------------------------------------------------
    fn at_angle(angle: i32) -> u16 {
        let mapped = (angle * (480 - 90) / 180 + 90).clamp(90, 480);
        mapped as u16
    }
    
    //defaulting all the servos , to display startup mode
    pca.set_channel_on_off(Channel::C0, 0, at_angle(0)).await.unwrap();      //servo min is 90       
    pca.set_channel_on_off(Channel::C1, 0, at_angle(180)).await.unwrap();// upside down
    pca.set_channel_on_off(Channel::C2, 0, at_angle(0)).await.unwrap();   
    

    // =================================================================
    // taking btn inputs to calibrate neck functions
    //===================================================================
    let l_up    = Input::new(peripherals.GPIO14, InputConfig::default().with_pull(Pull::Up));  
    let l_down  = Input::new(peripherals.GPIO12, InputConfig::default().with_pull(Pull::Up));
    let r_up    = Input::new(peripherals.GPIO26, InputConfig::default().with_pull(Pull::Up));  
    let r_down  = Input::new(peripherals.GPIO27, InputConfig::default().with_pull(Pull::Up));  
    
    let mut l_neck_angle : i32 = 0.clamp(0,180) ;
    let mut r_neck_angle : i32 = 0.clamp(0,180);  // arm opposite to the other
    

    //==========================================================================================
    //      talk for mouth 
    //==========================================================================================
    
    fn mouth(x:i32)->i32{
        x+90
    }
    pca.set_channel_on_off(Channel::C3, 0, at_angle(mouth(0))).await.unwrap();
    Timer::after(Duration::from_secs(2)).await;

    pca.set_channel_on_off(Channel::C2, 0, at_angle(mouth(45))).await.unwrap();
    Timer::after(Duration::from_secs(2)).await;

    pca.set_channel_on_off(Channel::C2,0, at_angle(mouth(90))).await.unwrap();
    Timer::after(Duration::from_secs(2)).await;
    
    pca.set_channel_on_off(Channel::C2,0, at_angle(mouth(45))).await.unwrap();
    Timer::after(Duration::from_secs(2)).await;
    
    pca.set_channel_on_off(Channel::C2,0, at_angle(mouth(90))).await.unwrap();
    Timer::after(Duration::from_secs(2)).await;
        
        





    loop {
        if l_up.is_low() {
            l_neck_angle+=5; 
            info!("l+5");
        }
        if l_down.is_low() {
            l_neck_angle-=5;     
            info!("l-5");
        }
        if r_up. is_low() {
            r_neck_angle+=5;
            info!("r+5"); 
        }
        if r_down. is_low() {
            r_neck_angle-=5;
            info!("r-5");
        }

        //- servo channel 0 & 1 for neck tilts and lifts
        pca.set_channel_on_off(Channel::C0, 0, at_angle(l_neck_angle)).await.unwrap();
        pca.set_channel_on_off(Channel::C1, 0, at_angle(r_neck_angle)).await.unwrap();
        
        Timer::after(Duration::from_millis(25)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
