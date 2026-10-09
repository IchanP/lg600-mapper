use evdev::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device: Device = Device::open("/dev/input/event10")?;

    loop {
        for event in device.fetch_events().unwrap() {
            println!("{:?}", event);
        }
    }
}
