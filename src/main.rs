use evdev::KeyCode;
use zbus::{Connection, Proxy, zvariant::OwnedObjectPath, zvariant::OwnedValue, zvariant::Value};

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

    println!("{:?}", name);
    println!("{:?}", model);

    let profile = make_proxy(&connection, &profiles[1], "Profile").await?;
    // let active: bool = profile.get_property("IsActive").await?;
    // println!("{:?}", active);

    let buttons: Vec<OwnedObjectPath> = profile.get_property("Buttons").await?;

    for button in buttons[8..20].into_iter() {
        let butt = make_proxy(&connection, &button, "Button").await?;

        let index: u32 = butt.get_property("Index").await?;
        let (ty, val): (u32, OwnedValue) = butt.get_property("Mapping").await?;
        // unwrap the extra variant layer
        let v: &Value = match &*val {
            Value::Value(inner) => inner,
            other => other,
        };

        match v {
            Value::U32(n) => println!("button {index}: type {ty}, value {n}"),
            Value::Array(arr) => {
                let keys: Vec<String> = arr
                    .iter()
                    .filter_map(|e| match e {
                        Value::Structure(s) => match s.fields() {
                            [Value::U32(1), Value::U32(k)] => {
                                Some(format!("{:?} ({k})", KeyCode::new(*k as u16)))
                            }
                            _ => None,
                        },
                        _ => None,
                    })
                    .collect();
                println!("button {index}: macro {}", keys.join(" + "));
            }
            other => println!("button {index}: type {ty}, {other:?}"),
        }
    }

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
