use crate::tasks::servo::SERVO;
use pwm_pca9685::Channel;
use embassy_time::{Duration, Timer};
use esp_println::println;
    

pub async fn set_pwm(servo_addr:u8 , ticks:u8){
    let servo = match servo_addr{
            0 => Channel::C0,  //head pan
            1 => Channel::C1,   // head pitch
            2 => Channel::C2,
            3 => Channel::C3,
            4 => Channel::C4,
            5 => Channel::C5,
            _ => {  println!("invalid servo address");
                    return  
                }
        };
    let mut guard = SERVO.lock().await;
    if let Some(s) = guard.as_mut() {
        println!("now servo servo {} is at {}",servo_addr,ticks);
        s.pca.set_channel_on_off(servo, 0, ticks as u16 *2).await.unwrap();
                                            // as the server uses u8 max combo is upto 256 
                                            //  we need upto 480 so we double
                                            // and as this will be mostlu used for debugging ,
                                            // it doesn't really matter
    }
}