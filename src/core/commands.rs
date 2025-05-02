use std::sync::Arc;
use bluer::gatt::local::{CharacteristicWriteFun, CharacteristicReadFun};
use futures::FutureExt;
use tokio::sync::Mutex;


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