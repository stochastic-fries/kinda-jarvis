use embassy_time::{Duration, Timer};
use embassy_net::{Config as NetConfig, Runner, StackResources, Stack};
use esp_println::println;
use esp_radio::wifi::{Config as WifiConfig, ControllerConfig, Interface, WifiController, sta::StationConfig};
use static_cell::StaticCell;
use alloc::string::ToString;

const SSID:     &str = "";  // trying not to upload this on github 
const PASSWORD: &str = "";  // please don't hunt for these in future commits :)

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

    // to do - socket client setup 

    loop {
        Timer::after(Duration::from_secs(1)).await;
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