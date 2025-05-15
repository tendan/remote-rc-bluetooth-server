use rppal::pwm::Channel;

// BCM pin numbering is used
pub const ACCELERATOR_GPIO: u8 = 23;
pub const SERVO_GPIO: Channel = Channel::Pwm0; // PWM PIN

pub const PERIOD_MS: u64 = 20;
pub const PULSE_MIN_US: u64 = 1200;
pub const PULSE_NEUTRAL_US: u64 = 1500; // Center the wheel
pub const PULSE_MAX_US: u64 = 1800;

pub const MAX_DEGREE: u8 = 180;