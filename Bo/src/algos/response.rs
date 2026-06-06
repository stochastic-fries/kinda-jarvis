use crate::tasks::servo::SERVO;
use pwm_pca9685::Channel;
use embassy_time::{Duration, Timer};
use esp_println::println;

pub async fn nod() {
    let mut guard = SERVO.lock().await;
    if let Some(s) = guard.as_mut() {
        // direct pwm access
        let mut x = 260;
        println!("noded");
        for i in 1..60{
            s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
            x-=1;
            Timer::after(Duration::from_millis(10)).await;

        }
        for i in 1..120{
            s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
            x+=1;
            Timer::after(Duration::from_millis(10)).await;

        }
        for i in 1..60{
            s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
            x-=1;
            Timer::after(Duration::from_millis(10)).await;

        }
    }
}
