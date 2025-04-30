use bluer::{
    gatt::local::{
        Application, Characteristic, CharacteristicControlHandle,
        CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicWrite,
        Service
    }, 
    Uuid
};
use crate::config::uuid::{
    REQUEST_RESPONSE_CHARACTERISTIC_UUID,
    CONTROL_SYSTEM_CHARACTERISTIC_UUID,
    SERVICE_UUID
};

pub fn prepare_application(control_handle: &CharacteristicControlHandle) -> Application {
    let characteristics: Vec<Characteristic> = vec![
        Characteristic {
            uuid: Uuid::parse_str(REQUEST_RESPONSE_CHARACTERISTIC_UUID).unwrap(),
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
            control_handle,
            ..Default::default()
        },
        Characteristic {
            uuid: Uuid::parse_str(CONTROL_SYSTEM_CHARACTERISTIC_UUID).unwrap(),
            write: Some(CharacteristicWrite {
                write_without_response: false,
                method: bluer::gatt::local::CharacteristicWriteMethod::Io,
                ..Default::default()
            }),
            notify: Some(CharacteristicNotify {
                notify: true,
                indicate: true,
                method: CharacteristicNotifyMethod::Io,
                ..Default::default()
            }),
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