use std::sync::Arc;

use bluer::{Adapter, AdapterEvent};
use bluer::AdapterProperty::{
    Discoverable,
    Discovering,
    Pairable,
    Powered,
};
use log::{info, debug};
use zbus::fdo::PropertiesChanged;
use zbus::{Connection, MatchRule, MessageStream};
use futures::{Stream, StreamExt, TryStreamExt};

pub async fn monitor_disconnects() -> zbus::Result<()> {
    let conn = Connection::system().await?;
    
    
    let rule = MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("org.freedesktop.DBus.Properties")?
        .member("PropertiesChanged")?
        .add_arg("org.bluez.Device1")?
        .build();

    let mut stream = MessageStream::for_match_rule(
        rule, &conn, Some(1)).await?;
    
    println!("🔍 Monitoring BLE client connections...");

    while let Some(msg) = stream.try_next().await? {
        let Some(signal) = PropertiesChanged::from_message(msg.clone()) else { continue; };
        
        let args = signal.args()?;
        let Some(val) = args.changed_properties().get("Connected") else { continue; };

        let Ok(is_connected) = val.downcast_ref::<bool>() else { continue; };

        let device_path = msg.header()
            .path().map(|p| p.to_string()).unwrap_or_else(|| "<unknown>".into());
        
        if is_connected {
            info!("Device connected: {device_path}");
        } else {
            info!("Device disconnected: {device_path}");
        }
    }

    Ok(())
}

// pub async fn event_listener(adapter: Arc<Adapter>) -> bluer::Result<()> {
//     let mut events = adapter.events().await?;
//     info!("Event listener instantiated");
//     while let Some(evt) = events.next().await {
//         match evt {
//             AdapterEvent::DeviceAdded(addr) => info!("New device: {}", addr),
//             AdapterEvent::DeviceRemoved(addr) => info!("Device removed: {}", addr),
//             AdapterEvent::PropertyChanged(Discoverable(val)) => info!("Discoverable: {}", val),
//             AdapterEvent::PropertyChanged(Discovering(val)) => info!("Discovering: {}", val),
//             AdapterEvent::PropertyChanged(Pairable(val)) => info!("Pairable: {}", val),
//             AdapterEvent::PropertyChanged(Powered(val)) => info!("Powered: {}", val),
//             AdapterEvent::PropertyChanged(_) => {debug!("Anything else changed in adapter")}
//         }
//     }

//     Ok(())
// }