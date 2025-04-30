mod config;
mod core;
use core::{advertise::create_advertisement, app::prepare_application, handlers::event_loop};
//use futures::pin_mut;
use bluer::gatt::local::characteristic_control;


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

    let (mut dummy_char_control, dummy_char_handle) = characteristic_control();
    let (mut controls_char_control, controls_char_handle) = characteristic_control();
    //pin_mut!(dummy_char_control);
    //pin_mut!(controls_char_control);
    let app = prepare_application(dummy_char_handle, controls_char_handle);

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Echo service ready. Press enter to quit.");

    event_loop(&mut dummy_char_control, &mut controls_char_control).await?;

    drop(app_handle);

    println!("Removing advertisement");
    drop(adv_handle);

    Ok(())
}
