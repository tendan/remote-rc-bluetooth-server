mod config;
mod core;
use core::{advertise::create_advertisement, app::prepare_application, handlers::event_loop};
//use futures::pin_mut;
use bluer::gatt::local::characteristic_control;
use tokio::io::{AsyncBufReadExt, BufReader};


// Need to perform "rfkill unlock" for proper work
#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    //println!("Pairable: {:?}", adapter.is_pairable().await?);
    adapter.set_powered(true).await?;

    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = create_advertisement();

    let adv_handle = adapter.advertise(le_advertisement).await?;
    
    //println!("Pairable: {:?}", adapter.is_pairable().await?);

    let (mut dummy_char_control, dummy_char_handle) = characteristic_control();
    let (mut controls_char_control, controls_char_handle) = characteristic_control();
    //pin_mut!(dummy_char_control);
    //pin_mut!(controls_char_control);
    let app = prepare_application(dummy_char_handle, controls_char_handle);

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;
    //event_loop(&mut dummy_char_control, &mut controls_char_control).await?;

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);

    Ok(())
}
