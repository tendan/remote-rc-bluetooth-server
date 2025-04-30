use bluer::{
    adv::Advertisement,
    Uuid
};

use crate::config::uuid::SERVICE_UUID;

pub fn create_advertisement() -> Advertisement {
    Advertisement  {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: vec![Uuid::parse_str(SERVICE_UUID).unwrap()].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("Remote RC BT".to_string()),
        ..Default::default()
    }
}