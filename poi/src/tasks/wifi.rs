use embassy_net::{tcp::TcpSocket, IpListenEndpoint, Stack};
use embassy_time::{Duration, Timer};
use esp_println::println;

#[embassy_executor::task]
pub async fn wifi_task(stack: Stack<'static>) {
    // wait for link
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
    println!("WiFi link up!");

    // get the ip
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    // TCP server on port 8080
    let mut rx_buf = [0u8; 4096];
    let mut tx_buf = [0u8; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);
        socket.set_timeout(Some(Duration::from_secs(30)));

        println!("waiting for connection on port 8080...");
        if socket.accept(IpListenEndpoint { addr: None, port: 8080 }).await.is_err() {
            continue;
        }
        println!("client connected!");

        let mut chunk = [0u8; 256];
        let mut total = 0usize;
        loop {
            match socket.read(&mut chunk).await {
                Ok(0) => {
                    println!("client disconnected, received {} bytes total", total);
                    break;
                }
                Ok(n) => {
                    total += n;
                    if total % 4096 == 0 {
                        println!("received {} bytes so far...", total);
                    }
                }
                Err(e) => {
                    println!("tcp error: {:?}", e);
                    break;
                }
            }
        }
    }
}