use embassy_time::{Duration, Timer};
use embassy_net::{Config as NetConfig, Runner, StackResources, Stack};
use esp_println::println;
use esp_radio::wifi::{Config as WifiConfig, ControllerConfig, Interface, WifiController, sta::StationConfig};
use static_cell::StaticCell;
use alloc::string::ToString;
use embassy_net::{tcp::TcpSocket, IpEndpoint, IpAddress};
use embedded_io_async::Write;


const SSID:     &str = "";  // trying not to upload this on github 
const PASSWORD: &str = "";  // please don't hunt for these in future commits :)

const LAPTOP_IP: IpAddress  = IpAddress::v4(192, 168, 29, 183); 
const WS_PORT:   u16        = 9090;

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();


//_______________________________________________________________________________________________________
//                          -- Helper functions --
//                          _______________________
fn decode_ws_frame<'a>(buf: &'a mut [u8], n: usize) -> Option<&'a [u8]> {
    if n < 6 { return None; }
    
    // payload length is in the lower 7 bits of byte 1
    let len = (buf[1] & 0x7F) as usize;
    
    // mask key is always bytes 2,3,4,5
    let mask = [buf[2], buf[3], buf[4], buf[5]];
    
    // unmask the payload starting at byte 6
    for i in 0..len {
        buf[6 + i] ^= mask[i % 4];
    }
    
    // return the unmasked payload
    Some(&buf[6..6 + len])
}


//_________________________________________________________________________________________________________

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

      
        let mut resp = [0u8; 512];
        match socket.read(&mut resp).await {
            Ok(n) => {
                
                let s = core::str::from_utf8(&resp[..n]).unwrap_or("?");
                println!("server response: {}", s);
                if !s.contains("\r\n\r\n") {
                    println!("WS upgrade failed!");
                    continue;
                }
                if !core::str::from_utf8(&resp[..n]).unwrap_or("").contains("101") {
                    println!("WS upgrade failed!");
                    continue;
                }
                println!("WebSocket connected!");
            } 
            Err(_) => continue,
        }
    
        let mut buf = [0u8; 256];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => { println!("disconnected"); break; }
                Ok(n) => {
                    if let Some(payload) = decode_ws_frame(&mut buf, n) {
                        if payload.len() >= 4 {
                            let cat = payload[0];
                            let cmd = payload[1];
                            let p1  = payload[2];
                            let p2  = payload[3];
                            match (cat, cmd) {
                                (0x01, 0x01) => println!("nod"),
                                _ => println!("unknown: {:#x} {:#x}", cat, cmd),
                            }
                        }
                    }
                }
                Err(e) => { println!("error: {:?}", e); break; }
            }
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