extern crate i2cdev;
extern crate i2c_pca9685;

use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};

const DEFAULT_PCA9685_ADDRESS: u16 = 0x40;
const SERVO_MIN: u8 = 65;
const SERVO_MAX: u8 = 220;

struct Joint<'a> {
  servos: &'a i2c_pca9685::PCA9685<i2cdev::linux::LinuxI2CDevice>,
  pin: u8,
  min: u8,
  max: u8,
  pos: u8,
}
impl <'a>Joint<'a> {
  fn new(servos: &i2c_pca9685::PCA9685<i2cdev::linux::LinuxI2CDevice>, pin: u8, min: u8, max: u8) -> Joint {
    let pos = max - min / 2;
    Joint { servos, pin, min, max, pos }
  }

  fn center(&mut self) {
    self.mv(self.max - self.min / 2);
  }

  fn mv(&mut self, pos: u8) -> () {
    if pos > self.max || pos < self.min { return println!("Cannot move beyond limits") }
    self.pos = pos
    // Send pwm to `pin`
  }
}

enum LegType { Front, Middle, Back }

// Each leg for one side will have different motions asociated with a step... I.e. back legs will
// behave differently from front legs
struct Leg<'a> {
  leg_type: LegType,
  pelvis:   Joint<'a>,
  knee:     Joint<'a>,
  ankle:    Joint<'a>
}

impl<'a> Leg<'a> {
  fn new(leg_type: LegType, mut pelvis: Joint<'a>, mut knee: Joint<'a>, mut ankle: Joint<'a>) -> Leg<'a> {
    pelvis.center();
    knee.center();
    ankle.center();
    Leg { leg_type, pelvis, knee, ankle }
  }

  fn step(&mut self, dir: u16, speed: u64) -> () {
    // match leg_type
    println!("Doing step!");
    match self.leg_type {
      LegType::Front => {
        // Move knee up
        self.knee.mv(90);
        // Compensate ankle
        self.ankle.mv(180);
        // Turn pelvis max
        self.pelvis.mv(self.pelvis.max);
        println!("Moved front leg {0}, {1}, {2}", self.pelvis.pos, self.knee.pos, self.ankle.pos );
        thread::sleep(time::Duration::from_millis(speed));
        // Move knee down
        self.knee.mv(45);
        // Compensate ankle
        self.ankle.mv(180 - 45);
        // Turn pelvis min
        self.pelvis.mv(self.pelvis.min);
        println!("Moved front leg {0}, {1}, {2}", self.pelvis.pos, self.knee.pos, self.ankle.pos );
        thread::sleep(time::Duration::from_millis(speed));

      }
      _ => {
        println!("Not implemented!");
      }
    }
    println!("stepped!");

  }

  fn rest(mut self) -> () {
    // Go into resting position for this leg
    self.pelvis.center();
    self.knee.center();
    self.ankle.center();
  }

}

struct Brain<'a> {
  fl: Leg<'a>,
  // fr: Leg,
  // ml: Leg,
  // mr: Leg,
  // bl: Leg,
  // br: Leg
}
impl<'a> Brain<'a> {
  fn new(servos: &i2c_pca9685::PCA9685<i2cdev::linux::LinuxI2CDevice>) -> Brain {
    println!("New brain!");

    let ankle  = Joint::new(&servos, 0, 0, 180);
    let knee   = Joint::new(&servos, 0, 0, 180);
    let pelvis = Joint::new(&servos, 0, 0, 180);

    let fl = Leg::new(
      LegType::Front,
      ankle,
      knee,
      pelvis,
    );

    Brain {
      fl: fl
    }
  }

  fn walk(&mut self, dir: u16, speed: u64) -> () {
    self.fl.step(dir, speed);
  }
}

fn main() {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_walk() {
    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();
    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();

    let mut brain = Brain::new(&servos);
    brain.walk(0, 1000);

  }
}
