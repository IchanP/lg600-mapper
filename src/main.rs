use zbus::{Connection, zvariant::OwnedObjectPath};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO do we need this?
    let connection = Connection::system().await?;
    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.ratbag1",
        "/org/freedesktop/ratbag1",
        "org.freedesktop.ratbag1.Manager",
    )
    .await?;

    let list: Vec<OwnedObjectPath> = proxy.get_property("Devices").await?;

    // TODO the path is derived from the list, we need to grab that somehow.
    let device_proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.ratbag1",
        "/org/freedesktop/ratbag1/device/hidraw7",
        "org.freedesktop.ratbag1.Device",
    )
    .await?;

    let name: String = device_proxy.get_property("Name").await?;
    let model: String = device_proxy.get_property("Model").await?;

    println!("{:?}", name);
    println!("{:?}", model);
    Ok(())
}
