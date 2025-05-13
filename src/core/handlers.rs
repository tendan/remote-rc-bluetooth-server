use bluer::gatt::{
        local::{
            CharacteristicControl, CharacteristicControlEvent
        },
        CharacteristicReader, CharacteristicWriter
    };
use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};
use futures::{future, StreamExt};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader}, sync::Mutex, time::interval
};
use core::error::Error;
use log::{error, info};

use super::hardware::{accelerate, stop_acceleration, steer};
// use crate::core::commands::send_dummy_command;

// pub async fn event_loop(
//     char_control: &mut CharacteristicControl,
//     controls_char_control: &mut CharacteristicControl
// ) -> bluer::Result<()> {
//     let stdin = BufReader::new(tokio::io::stdin());
//     let mut lines = stdin.lines();

//     let mut value: Vec<u8> = vec![0x10, 0x01, 0x01, 0x10];
//     let mut read_buf = Vec::new();
//     let mut reader_opt: Option<CharacteristicReader> = None;
//     let mut writer_opt: Option<CharacteristicWriter> = None;
//     let mut interval = interval(Duration::from_secs(1));

//     loop {
//         tokio::select! {
//             _ = lines.next_line() => break Ok(()),
//             evt = char_control.next() => {
//                 match evt {
//                     Some(CharacteristicControlEvent::Write(req)) => {
//                         println!("Accepting write event with MTU {} from {}", req.mtu(), req.device_address());
//                         read_buf = vec![0; req.mtu()];
//                         reader_opt = Some(req.accept()?);
//                     },
//                     Some(CharacteristicControlEvent::Notify(notifier)) => {
//                         println!("Accepting notify request event with MTU {} from {}", notifier.mtu(), notifier.device_address());
//                         writer_opt = Some(notifier);
//                     },
//                     None => break Ok(()),
//                 }
//             }
//             //_ = interval.tick() => { send_dummy_command(&mut writer_opt).await }
//             read_res = read_buffer(&mut reader_opt, &mut read_buf) => {
//                 match read_res {
//                     Ok(0) => {
//                         println!("Write stream ended");
//                         reader_opt = None;
//                     }
//                     Ok(n) => {
//                         value = read_buf[0..n].to_vec();
//                         println!("Write request with {} bytes: {:x?}", n, &value);
//                     }
//                     Err(err) => {
//                         println!("Write stream error: {}", &err);
//                         reader_opt = None;
//                     }
//                 }
//             }
//         }
//     }
// }

// async fn read_buffer(reader_opt: &mut Option<CharacteristicReader>, read_buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
//     match reader_opt {
//         Some(reader) => reader.read(read_buf).await,
//         None => future::pending().await,
//     }
// }

pub fn parse_command(command: &Vec<u8>/* , current_acc: Arc<AtomicBool> */) -> bluer::Result<()> {
    match command[..] {
        [0x01, _, 0x00, b] => {
            // this will be removed when proper method will exist
            accel_handle(b);
            Ok(())
        },
        [0x01, _, 0x01, d] => {
            // this will be removed when proper method will exist
            steering_handle(d);
            Ok(())
        },
        // [0x03, _, _, b] => {
        //     //println!("Thumb position: {:x?}", b);
        //     Ok(())
        // }
        _ => Ok(())
    }
}

fn accel_handle(id: u8) {
    match id {
        0 => stop_acceleration(),
        1 => accelerate(),
        2 => error!("TODO: Brake on"),
        3 => error!("TODO: Brake off"),
        _ => println!("Unknown command")
    }
}

fn steering_handle(degrees: u8) {
    if degrees > 180 {
        error!("Invalid degree value");
        return
    }
    steer(degrees);
}

pub fn on_disconnect(/* current_acc: Arc<AtomicBool> */) {
    info!("Stopping vehicle due to disconnection");
    stop_acceleration(/* current_acc.clone() */);
}