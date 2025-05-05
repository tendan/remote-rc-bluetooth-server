use bluer::{
    gatt::local::{
        Application, Characteristic, CharacteristicControlHandle,
        CharacteristicNotify, CharacteristicNotifyMethod,
        CharacteristicRead, CharacteristicWrite,
        CharacteristicWriteMethod, Service
    }, 
    Uuid
};
use std::{sync::Arc, time::Duration};
use futures::FutureExt;
use tokio::{
    sync::Mutex,
    time::sleep,
};
use crate::config::uuid::{
    REQUEST_RESPONSE_CHARACTERISTIC_UUID,
    CONTROL_SYSTEM_CHARACTERISTIC_UUID,
    SERVICE_UUID
};
use super::commands::{control_command, receive_dummy_command, send_dummy_command};

// TODO: Refactor it into macro
pub fn prepare_application(
    dummy_control_handle: CharacteristicControlHandle,
    control_system_control_handle: CharacteristicControlHandle
) -> Application {
    let value = Arc::new(Mutex::new(vec![0x10, 0x00, 0x00, 0x00]));
    
    //let dummy_value_read = value.clone();
    //let dummy_value_write = value.clone();
    let dummy_value = value.clone();

    let control_value_write = value.clone();
    let control_value_notify = value.clone();

    let characteristics: Vec<Characteristic> = vec![
        Characteristic {
            uuid: Uuid::parse_str(REQUEST_RESPONSE_CHARACTERISTIC_UUID).unwrap(),
            // Sending request to client about connection status
            // Notify client about connection status
            notify: Some(CharacteristicNotify {
                indicate: true,
                notify: false,
                method: CharacteristicNotifyMethod::Fun(Box::new(move |mut notifier| {
                    let value = control_value_notify.clone();
                    async move {
                        tokio::spawn(async move {
                            if notifier.confirming() {
                                println!("Control system's indicate session start");
                            } else {
                                println!("Control system's notification session start");
                            }
                            loop {
                                {
                                    let mut value = value.lock().await;
                                    println!("Notifying with value {:x?}", &*value);
                                    if let Err(err) = notifier.notify(value.to_vec()).await {
                                        println!("Notification error: {}", &err);
                                        break;
                                    }
                                    println!("Decrementing each element by one");
                                    for v in &mut *value {
                                        *v = v.saturating_sub(1);
                                    }
                                }
                                sleep(Duration::from_secs(5)).await;
                            }
                            println!("Control system's notification session stop");
                        });
                    }
                    .boxed()
                })),
                ..Default::default()
            }),
            // Reading response from client
            write: Some(CharacteristicWrite {
                write_without_response: true,
                method: CharacteristicWriteMethod::Fun(receive_dummy_command(dummy_value.clone())),
                ..Default::default()
            }),
            control_handle: dummy_control_handle,
            ..Default::default()
        },
        Characteristic {
            uuid: Uuid::parse_str(CONTROL_SYSTEM_CHARACTERISTIC_UUID).unwrap(),
            // Reading response from client
            write: Some(CharacteristicWrite {
                write_without_response: true,
                method: CharacteristicWriteMethod::Fun(control_command(control_value_write)),
                ..Default::default()
            }),
            // read: Some(CharacteristicRead {
            //     read: true,
            //     fun: send_dummy_command(dummy_value.clone()),
            //     ..Default::default()
            // }),
            control_handle: control_system_control_handle,
            ..Default::default()
        }
    ];
    Application {
        services: vec![Service {
            uuid: Uuid::parse_str(SERVICE_UUID).unwrap(),
            primary: true,
            characteristics,
            ..Default::default()
        }],
        ..Default::default()
    }
}