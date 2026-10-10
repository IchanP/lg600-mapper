use std::io::Error;
use std::io::ErrorKind;

use evdev::{Device, InputEvent, uinput::VirtualDevice};

const VRT_DEVICE_NAME: &str = "LG600_VRT_KEYBOARD";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device: Device = Device::open("/dev/input/event10")?;
    device.grab()?;

    let mut vrt_device: VirtualDevice = create_virt_device(&device)?;

    loop {
        for event in device.fetch_events().unwrap() {
            let mod_event = InputEvent::new(event.event_type().0, event.code(), event.value());
            // TODO maybe handle errors somehow, also batch events?
            let _ = vrt_device.emit(&[mod_event]);
        }
    }
}

fn create_virt_device(real_device: &Device) -> Result<VirtualDevice, Error> {
    let keys = real_device
        .supported_keys()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to get keys from device."))?;

    let builder = evdev::uinput::VirtualDevice::builder()?
        .name(VRT_DEVICE_NAME)
        .with_keys(keys)?;

    builder.build()
}
