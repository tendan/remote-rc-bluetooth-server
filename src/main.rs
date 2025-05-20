mod config;
mod core;
use core::{adapter::monitor_disconnects, advertise::create_advertisement, app::prepare_application};
use std::sync::Arc;
//use futures::pin_mut;
use bluer::gatt::local::characteristic_control;
use log::{error, info};
use tokio::io::{AsyncBufReadExt, BufReader};


// Need to perform "rfkill unlock" for proper work
#[tokio::main]
async fn main() -> bluer::Result<()> {
    env_logger::init();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    //println!("Pairable: {:?}", adapter.is_pairable().await?);
    adapter.set_powered(true).await?;

    info!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = create_advertisement();

    let adv_handle = adapter.advertise(le_advertisement).await?;
    
    //println!("Pairable: {:?}", adapter.is_pairable().await?);

    let (/* mut dummy_char_control */_, dummy_char_handle) = characteristic_control();
    let (/* mut controls_char_control */_, controls_char_handle) = characteristic_control();
    //pin_mut!(dummy_char_control);
    //pin_mut!(controls_char_control);
    let app = prepare_application(dummy_char_handle, controls_char_handle);

    let app_handle = adapter.serve_gatt_application(app).await?;


    // Detects disconnect on Bluetooth adapter level
    tokio::spawn(async move {
        if let Err(e) = monitor_disconnects().await {
            error!("Error occured in disconnects monitor: {}", e);
        }
    });

    info!("Application service ready. Press enter to quit.");

    // TODO: External way to stop the application (maybe with machine shutdown)
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;
    //event_loop(&mut dummy_char_control, &mut controls_char_control).await?;

    info!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);

    Ok(())
}
