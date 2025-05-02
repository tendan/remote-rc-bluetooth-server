use std::sync::Arc;
use bluer::gatt::{local::CharacteristicWriteFun, CharacteristicWriter};
use futures::FutureExt;
use tokio::{io::AsyncWriteExt, sync::Mutex};

pub async fn send_dummy_command(writer_opt: &mut Option<CharacteristicWriter>) {
    println!("Sending dummy command");
    let value = vec![0x01, 0x00, 0x00, 0x00];
    println!("Value is {:x?}", &value);
    if let Some(writer) = writer_opt.as_mut() {
        println!("Notifying with value {:x?}", &value);
        if let Err(err) = writer.write(&value).await {
            println!("Notification stream error: {}", &err);
            *writer_opt = None;
        }
    }
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