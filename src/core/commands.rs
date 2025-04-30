use bluer::gatt::CharacteristicWriter;
use tokio::io::AsyncWriteExt;

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