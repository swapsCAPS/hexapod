extern crate i2cdev;
extern crate i2c_pca9685;

use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};

const DEFAULT_PCA9685_ADDRESS: u16 = 0x40;

enum LegType { Front, Middle, Back }

fn step(servos: &mut i2c_pca9685::PCA9685<i2cdev::linux::LinuxI2CDevice>, leg_type: LegType, dir: u16, delay: u64) -> () {
  println!("Doing step!");
  match leg_type {
    LegType::Front => {
      servos.set_pwm(0, 0, 100).unwrap();
      thread::sleep(time::Duration::from_millis(delay));
      thread::sleep(time::Duration::from_millis(delay));
    }
    _ => {
      println!("Not implemented!");
    }
  }
  println!("stepped!");
}

fn main() {
  let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();
  let mut servos = PCA9685::new(i2cdevice).unwrap();
  servos.set_pwm_freq(60.0).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_walk() {
    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();
    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();

    step(servos, LegType::Front, 0, 1000)
  }
}
