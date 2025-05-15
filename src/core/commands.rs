use std::sync::{atomic::AtomicBool, Arc};
use bluer::gatt::local::{CharacteristicWriteFun, CharacteristicReadFun};
use futures::FutureExt;
use log::error;
use tokio::sync::Mutex;

use crate::core::handlers::{parse_command, CommandHandleError};


pub fn send_dummy_command(previous_value: Arc<Mutex<Vec<u8>>>) -> CharacteristicReadFun {
    Box::new(move |req| {
        let value = previous_value.clone();
        async move {
            let value = value.lock().await.clone();
            println!("Dummy read request {:?}: {:x?}", &req, &value);
            Ok(value)
        }
        .boxed()
    })
}

pub fn receive_dummy_command(previous_value: Arc<Mutex<Vec<u8>>>) -> CharacteristicWriteFun {
    Box::new(move |new_value, req| {
        let value = previous_value.clone();
        async move {
            println!("Dummy write request {:?}: FROM {:x?} TO {:x?}", &req, &value, &new_value);
            let mut value = value.lock().await;
            *value = new_value;
            Ok(())
        }
        .boxed()
    })
}

pub fn control_command(previous_value: Arc<Mutex<Vec<u8>>>/* , current_acc_state: Arc<AtomicBool> */) -> CharacteristicWriteFun {
    Box::new(move |new_value, req| {
        let value = previous_value.clone();
        //let current_acc = current_acc_state.clone();
        async move {
            //println!("Control system's write request {:?}", &req);
            if let Err(error) = parse_command(&new_value/* , current_acc */) {
                let error_message = match error {
                    CommandHandleError::TodoCommand => "TODO: Operation under",
                    CommandHandleError::HardwareError => "Hardware error",
                    CommandHandleError::UnknownCommand => "Unknown command",
                    CommandHandleError::InproperValue => "Invalid value was provided for this command"
                };
                error!("Command error: {}", error_message);
            }
            let mut value = value.lock().await;
            *value = new_value;
            Ok(())
        }
        .boxed()
    })
}