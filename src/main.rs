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

//const SERVICE_UUID: &str = "123e4567-e89b-12d3-a456-426614174000";

// Need to perform "rfkill unlock" for proper work
#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!("Pairable: {:?}", adapter.is_pairable().await?);
    adapter.set_powered(true).await?;

    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = Advertisement  {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: vec!["55174dae-1f5b-4943-82d1-a933cf19305e".parse().unwrap()].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("Remote RC BT".to_string()),
        ..Default::default()
    };

    let adv_handle = adapter.advertise(le_advertisement).await?;
    
    println!("Pairable: {:?}", adapter.is_pairable().await?);

    let (char_control, char_handle) = characteristic_control();
    let app = Application {
        services: vec![Service {
            uuid: Uuid::parse_str("55174dae-1f5b-4943-82d1-a933cf19305e").unwrap(),
            primary: true,
            characteristics: vec![Characteristic {
                uuid: Uuid::parse_str("2b022587-f2bc-4563-bd7a-0099940c533a").unwrap(),
                write: Some(CharacteristicWrite {
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
                notify: Some(CharacteristicNotify {
                    notify: true,
                    method: CharacteristicNotifyMethod::Io,
                    ..Default::default()
                }),
                control_handle: char_handle,
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Echo service ready. Press enter to quit.");

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut value: Vec<u8> = vec![0x10, 0x01, 0x01, 0x10];
    let mut read_buf = Vec::new();
    let mut reader_opt: Option<CharacteristicReader> = None;
    let mut writer_opt: Option<CharacteristicWriter> = None;
    let mut interval = interval(Duration::from_secs(1));
    pin_mut!(char_control);

    loop {
        tokio::select! {
            _ = lines.next_line() => break,
            evt = char_control.next() => {
                match evt {
                    Some(CharacteristicControlEvent::Write(req)) => {
                        println!("Accepting write event with MTU {} from {}", req.mtu(), req.device_address());
                        read_buf = vec![0; req.mtu()];
                        reader_opt = Some(req.accept()?);
                    },
                    Some(CharacteristicControlEvent::Notify(notifier)) => {
                        println!("Accepting notify request event with MTU {} from {}", notifier.mtu(), notifier.device_address());
                        writer_opt = Some(notifier);
                    },
                    None => break,
                }
            }
            _ = interval.tick() => {
                println!("Sending dummy command");
                value = vec![0x01, 0x00, 0x00, 0x00];
                println!("Value is {:x?}", &value);
                if let Some(writer) = writer_opt.as_mut() {
                    println!("Notifying with value {:x?}", &value);
                    if let Err(err) = writer.write(&value).await {
                        println!("Notification stream error: {}", &err);
                        writer_opt = None;
                    }
                }
            }
            read_res = async {
                match &mut reader_opt {
                    Some(reader) => reader.read(&mut read_buf).await,
                    None => future::pending().await,
                }
            } => {
                match read_res {
                    Ok(0) => {
                        println!("Write stream ended");
                        reader_opt = None;
                    }
                    Ok(n) => {
                        value = read_buf[0..n].to_vec();
                        println!("Write request with {} bytes: {:x?}", n, &value);
                    }
                    Err(err) => {
                        println!("Write stream error: {}", &err);
                        reader_opt = None;
                    }
                }
            }
        }
    }

    drop(app_handle);
    // println!("Press enter to quit");
    // let stdin = BufReader::new(tokio::io::stdin());
    // let mut lines = stdin.lines();
    // let _ = lines.next_line().await;

    println!("Removing advertisement");
    drop(adv_handle);

    Ok(())
}
