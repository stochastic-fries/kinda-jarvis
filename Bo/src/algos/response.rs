use crate::tasks::servo::SERVO;
use pwm_pca9685::Channel;
use embassy_time::{Duration, Timer};
use esp_println::println;

pub async fn nod() {
    let mut guard = SERVO.lock().await;
    if let Some(s) = guard.as_mut() {
        // direct pwm access
        let mut x = 285;
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

pub async fn now_twice(){
    let mut guard = SERVO.lock().await;
    if let Some(s) = guard.as_mut() {
        let mut x = 260;
        for i in 1..3{
            println!("noded");
            for i in 1..60{
                s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
                x-=1;
                Timer::after(Duration::from_millis(2)).await;

            }
            for i in 1..120{
                s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
                x+=1;
                Timer::after(Duration::from_millis(2)).await;

            }
            for i in 1..60{
                s.pca.set_channel_on_off(Channel::C1, 0, x).await.unwrap();
                x-=1;
                Timer::after(Duration::from_millis(5)).await;

            }
        }
    }

}



pub async fn deny_noding() {
    let mut guard = SERVO.lock().await;
    if let Some(s) = guard.as_mut() {
        // direct pwm access
        let mut x = 285;
        println!("noded");
        for i in 1..120{
            s.pca.set_channel_on_off(Channel::C0, 0, x).await.unwrap();
            x-=1;
            Timer::after(Duration::from_millis(5)).await;
        }
        for i in 1..240{
            s.pca.set_channel_on_off(Channel::C0, 0, x).await.unwrap();
            x+=1;
            Timer::after(Duration::from_millis(5)).await;
        }
        for i in 1..120{
            s.pca.set_channel_on_off(Channel::C0, 0, x).await.unwrap();
            x-=1;
            Timer::after(Duration::from_millis(5)).await;
        }

        s.pca.set_channel_on_off(Channel::C0, 0 , 285).await.unwrap();
    }
}