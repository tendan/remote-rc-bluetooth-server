use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use log::{info};
use rppal::{gpio::Gpio, pwm::{Channel, Polarity, Pwm}};
use crate::config::spec::*;

// pub fn set_accelerator(val: bool) -> Result<(), Box<dyn Error>> {
//     let mut accelerator_pin = Gpio::new()?.get(ACCELERATOR_GPIO)?.into_output();

//     let accelerate = Arc::new(AtomicBool::new(true));

//     accelerate.store(val, Ordering::SeqCst);

//     if accelerate.load(Ordering::SeqCst) {
//         accelerator_pin.set_high();
//     } else {
//         accelerator_pin.set_low();
//     }

//     Ok(())
// }

// Each function currently panics if something goes wrong with hardware

pub fn accelerate(/* current_acc_state: Arc<AtomicBool> */) -> Result<(), Box<dyn Error>> {
    let mut accelerator_pin = Gpio::new()?
            .get(ACCELERATOR_GPIO)?.into_output();
    info!("Accelerating");
    //current_acc_state.clone().store(true, Ordering::SeqCst);
    accelerator_pin.set_high();

    Ok(())
}

pub fn stop_acceleration(/* current_acc_state: Arc<AtomicBool> */) -> Result<(), Box<dyn Error>> {
    let mut accelerator_pin = Gpio::new()?
            .get(ACCELERATOR_GPIO)?.into_output();
    //current_acc_state.clone().store(false, Ordering::SeqCst);
    info!("Stopping the vehicle");
    accelerator_pin.set_low();

    Ok(())
}

pub fn steer(degrees: u8) -> Result<(), Box<dyn Error>> {
    let servo_pin = Pwm::with_period(
        SERVO_GPIO, 
        Duration::from_millis(PERIOD_MS), 
        Duration::from_micros(PULSE_MAX_US), 
        Polarity::Normal, 
        true
    )?;
    
    let pulse = (degrees as u64) * ((PULSE_MAX_US - PULSE_MIN_US) / (MAX_DEGREE as u64)) + PULSE_MIN_US;
    
    info!("Set servo to {} degrees", degrees);
    servo_pin.set_pulse_width(Duration::from_micros(pulse))?;

    Ok(())
}