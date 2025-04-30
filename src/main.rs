mod config;
mod core;
use core::{advertise::create_advertisement, app::prepare_application, commands::send_dummy_command, handlers::event_loop};
use std::time::Duration;
use bluer::{
    adv::Advertisement,
    gatt::{
        local::{
            characteristic_control, Application, Characteristic, CharacteristicControlEvent,
            CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicWrite, CharacteristicWriteMethod,
            Service,
        },
        CharacteristicReader, CharacteristicWriter,
    }, Uuid,
};
use futures::{future, pin_mut, StreamExt};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::{interval, sleep}
};

// Need to perform "rfkill unlock" for proper work
#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!("Pairable: {:?}", adapter.is_pairable().await?);
    adapter.set_powered(true).await?;

    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = create_advertisement();

    let adv_handle = adapter.advertise(le_advertisement).await?;
    
    println!("Pairable: {:?}", adapter.is_pairable().await?);

    let (char_control, char_handle) = characteristic_control();
    let app = prepare_application(&char_handle);

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Echo service ready. Press enter to quit.");

    event_loop(&char_control);

    drop(app_handle);

    println!("Removing advertisement");
    drop(adv_handle);

    Ok(())
}
