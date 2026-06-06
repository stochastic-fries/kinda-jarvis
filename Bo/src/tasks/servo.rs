use embassy_time::{Duration, Timer};
use esp_hal::i2c::master::I2c;
use esp_hal::Async;
use esp_println::println;
use pwm_pca9685::{Address, Channel, Pca9685};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

// servo channels
pub const CH_PAN:    Channel = Channel::C0;  // left/right  
pub const CH_TILT:   Channel = Channel::C1;  // up/down
pub const CH_ROTATE: Channel = Channel::C2;  // rotate


const SERVO_MIN: i32 = 90;
const SERVO_MAX: i32 = 480;


pub fn angle_to_pwm(angle: i32) -> u16 {
    (angle * (SERVO_MAX - SERVO_MIN) / 180 + SERVO_MIN)
    .clamp(SERVO_MIN, SERVO_MAX) as u16
}

//servo centers
// 285 is the servo center
const PAN_CENTER: u16 = 285;        //decrease to loook to right (his right), 
const TILT_CENTER: u16 = 260;       // decrease to look up 
const ROTATE_CENTER: u16 = 260;     // decrease to make robot tile left(his left)     

pub static SERVO: Mutex<CriticalSectionRawMutex, Option<ServoController>> = Mutex::new(None);
pub struct ServoController {
    pub pca: Pca9685<I2c<'static, Async>>,
}
unsafe impl Send for ServoController {}
impl ServoController {
    pub async fn new(i2c: I2c<'static, Async>) -> Self {
        let mut pca = Pca9685::new(i2c, Address::default()).unwrap();
        pca.set_prescale(121).await.unwrap(); // 50Hz
        pca.enable().await.unwrap();

        // boot position — all centered
        pca.set_channel_on_off(CH_TILT,   0 , TILT_CENTER).await.unwrap();
        pca.set_channel_on_off(CH_PAN,    0, PAN_CENTER).await.unwrap();
        pca.set_channel_on_off(CH_ROTATE, 0, ROTATE_CENTER).await.unwrap();

        println!("servos initialized!");
        Self { pca }
    }

    pub async fn set_angle(&mut self, channel: Channel, angle: i32) {
        let pwm = angle_to_pwm(angle);
        self.pca.set_channel_on_off(channel, 0, pwm).await.unwrap();
    }
}