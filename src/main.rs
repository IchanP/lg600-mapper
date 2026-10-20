use zbus::{Connection, Proxy, zvariant::OwnedObjectPath};

const DESTINATION: &str = "org.freedesktop.ratbag1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;
    let proxy = make_proxy(&connection, "/org/freedesktop/ratbag1", "Manager").await?;

    let list: Vec<OwnedObjectPath> = proxy.get_property("Devices").await?;

    // TODO the path is derived from the list, we need to grab that somehow.
    let device = make_proxy(&connection, &list[0], "Device").await?;

    let name: String = device.get_property("Name").await?;
    let model: String = device.get_property("Model").await?;
    let profiles: Vec<OwnedObjectPath> = device.get_property("Profiles").await?;

    // println!("{:?}", profiles);
    println!("{:?}", name);
    println!("{:?}", model);

    let profile = make_proxy(&connection, &profiles[2], "Profile").await?;

    let buttons: Vec<OwnedObjectPath> = profile.get_property("Buttons").await?;
    // println!("{:?}", buttons);
    let button_one = make_proxy(&connection, &buttons[1], "Profile").await?;

    let description = button_one.introspect().await?;
    println!("{:?}", description);

    Ok(())
}

async fn make_proxy(
    connection: &Connection,
    path: &str,
    interface: &str,
) -> zbus::Result<Proxy<'static>> {
    let mut inter: String = "org.freedesktop.ratbag1.".to_owned();
    inter.push_str(interface);
    Proxy::new(connection, DESTINATION, path.to_owned(), inter).await
}
