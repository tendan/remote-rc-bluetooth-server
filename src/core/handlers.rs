use bluer::{gatt::{
        local::{
            CharacteristicControl, CharacteristicControlEvent
        },
        CharacteristicReader, CharacteristicWriter
    }, Error};
use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};
use futures::{future, StreamExt};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader}, sync::Mutex, time::interval
};
use log::{error, info};

use crate::config::spec::MAX_DEGREE;

use super::hardware::{accelerate, stop_acceleration, steer};
// use crate::core::commands::send_dummy_command;

pub enum CommandHandleError {
    TodoCommand,
    UnknownCommand,
    InproperValue,
    HardwareError
}

type CommandResult<T> = Result<T, CommandHandleError>;

pub fn parse_command(command: &Vec<u8>/* , current_acc: Arc<AtomicBool> */) -> CommandResult<()> {
    match command[..] {
        [0x01, _, 0x00, b] => accel_handle(b),
        [0x01, _, 0x01, d] => steering_handle(d),
        _ => Ok(())
    }
}

fn accel_handle(id: u8) -> CommandResult<()> {
    match id {
        0 => stop_acceleration().map_err(|_| CommandHandleError::HardwareError),
        1 => accelerate().map_err(|_| CommandHandleError::HardwareError),
        2|3 => Err(CommandHandleError::TodoCommand), // TODO: Brake commands
        _ => Err(CommandHandleError::UnknownCommand)
    }
}

fn steering_handle(degrees: u8) -> CommandResult<()> {
    if degrees > MAX_DEGREE {
        return Err(CommandHandleError::InproperValue)
    }
    steer(degrees).map_err(|_| CommandHandleError::HardwareError)
}

pub fn on_disconnect(/* current_acc: Arc<AtomicBool> */) -> CommandResult<()> {
    info!("Stopping vehicle due to disconnection");
    stop_acceleration(/* current_acc.clone() */).map_err(|_| CommandHandleError::HardwareError)
}