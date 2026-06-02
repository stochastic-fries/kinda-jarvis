use embassy_time::{Duration, Timer};
use embassy_net::{Config as NetConfig, Runner, StackResources, Stack};
use esp_println::println;
use esp_radio::wifi::{Config as WifiConfig, ControllerConfig, Interface, WifiController, sta::StationConfig};
use static_cell::StaticCell;
use alloc::string::ToString;
use embassy_net::{tcp::TcpSocket, IpEndpoint, IpAddress};
use embedded_io_async::Write;


const SSID:     &str = "yashas";  // trying not to upload this on github 
const PASSWORD: &str = "$5YJshashi";  // please don't hunt for these in future commits :)

const LAPTOP_IP: IpAddress  = IpAddress::v4(192, 168, 29, 39); 
const WS_PORT:   u16        = 9090;

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

#[embassy_executor::task]
pub async fn wifi(stack:Stack<'static>){

    loop { if stack.is_link_up(){break;}
            Timer::after(Duration::from_millis(500)).await;}

    println!("WIFI LINK UP!!");

    //getting the esp's IP
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    // TCP connection 
    let mut rx_buf = [0u8; 1024];
    let mut tx_buf = [0u8; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);
        socket.set_timeout(Some(Duration::from_secs(10)));

        println!("connecting to laptop...");
        if socket.connect(IpEndpoint::new(LAPTOP_IP, WS_PORT)).await.is_err() {
            println!("connection failed, retrying...");
            Timer::after(Duration::from_secs(2)).await;
            continue;
        }
        println!("TCP connected!");

        //websocket
        let handshake = "GET / HTTP/1.1\r\nHost: 192.168.1.100:9090\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n";

        if socket.write_all(handshake.as_bytes()).await.is_err() {
            println!("handshake send failed");
            continue;
        }

        // read server response — should contain "101 Switching Protocols"
        let mut resp = [0u8; 256];
        match socket.read(&mut resp).await {
            Ok(n) => {
                let s = core::str::from_utf8(&resp[..n]).unwrap_or("?");
                println!("server response: {}", s);
                if !s.contains("101") {
                    println!("WS upgrade failed!");
                    continue;
                }
                println!("WebSocket connected!");
            }
            Err(_) => continue,
        }


        loop {
            Timer::after(Duration::from_secs(1)).await;
        }
    }
}



#[embassy_executor::task]
pub async fn wifi_connection_task(mut controller: WifiController<'static>) {
    println!("connecting to WiFi...");
    controller.connect_async().await.unwrap();
    println!("WiFi connected!");
    loop {
        Timer::after(Duration::from_secs(5)).await;
        controller.connect_async().await.ok();
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, Interface<'static>>) { //kinda daemon
    runner.run().await
}


pub fn init_wifi(
    wifi_peripheral: esp_hal::peripherals::WIFI<'static>,
) -> (WifiController<'static>, Runner<'static, Interface<'static>>, Stack<'static>) {
    let sta_config = StationConfig::default()
        .with_ssid(SSID)
        .with_password(PASSWORD.to_string());

    let controller_config = ControllerConfig::default()
        .with_initial_config(WifiConfig::Station(sta_config));

    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(wifi_peripheral, controller_config).unwrap();

    let resources = STACK_RESOURCES.init(StackResources::new());

    let (stack, runner) = embassy_net::new(
        interfaces.station,
        embassy_net::Config::dhcpv4(Default::default()),
        resources,
        12345u64, 
    );

    (wifi_controller, runner, stack)
}