use std::sync::Arc;

use bluer::{Adapter, AdapterEvent};
use bluer::AdapterProperty::{
    Discoverable,
    Discovering,
    Pairable,
    Powered,
};
use log::{info};
use futures::{Stream, StreamExt};


pub async fn event_listener(adapter: Arc<Adapter>) -> bluer::Result<()> {
    let mut events = adapter.events().await?;
    info!("Event listener instantiated");
    while let Some(evt) = events.next().await {
        match evt {
            AdapterEvent::DeviceAdded(addr) => info!("New device: {}", addr),
            AdapterEvent::DeviceRemoved(addr) => info!("Device removed: {}", addr),
            AdapterEvent::PropertyChanged(Discoverable(val)) => info!("Discoverable: {}", val),
            AdapterEvent::PropertyChanged(Discovering(val)) => info!("Discovering: {}", val),
            AdapterEvent::PropertyChanged(Pairable(val)) => info!("Pairable: {}", val),
            AdapterEvent::PropertyChanged(Powered(val)) => info!("Powered: {}", val),
            AdapterEvent::PropertyChanged(_) => {}
        }
    }

    Ok(())
}