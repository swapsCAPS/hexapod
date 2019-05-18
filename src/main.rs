extern crate i2cdev;
extern crate i2c_pca9685;

use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};
use std::cell::RefCell;


struct ServoWrapper {
servos: RefCell<i2c_pca9685::PCA9685<i2cdev::linux::LinuxI2CDevice>>,
}
impl ServoWrapper {
  fn new(address: u16) -> ServoWrapper {
    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", address).unwrap();
    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();
    ServoWrapper { servos: RefCell::new(servos) }
  }
}

struct Joint<'a> {
  servo_wrapper: &'a ServoWrapper,
  pin:           u8,
  min:           u16,
  max:           u16,
  pos:           u16,
  rat:           f32,
}
impl<'a> Joint<'a> {
  fn new(servo_wrapper: &ServoWrapper, pin: u8, min: u16, max: u16) -> Joint {
    let pos = max - min / 2;
    let rat = (max - min) as f32 / 180.0;
    Joint { servo_wrapper, pin, min, max, pos, rat }
  }

  fn mv(&mut self, degrees: u16) {
    let pos  = self.min + (self.rat * degrees as f32) as u16;
    self.pos = pos;
    self.servo_wrapper.servos.borrow_mut().set_pwm(self.pin, 0, pos).unwrap();
  }

  // Test the joint movement. Helpful when tweaking the PWM signal
  fn test(&self) {
    for j in self.min..self.max {
      println!("Pos: {}", j);
      self.servo_wrapper.servos.borrow_mut().set_pwm(self.pin, 1, j).unwrap();
      thread::sleep(time::Duration::from_millis(10));
    }

    thread::sleep(time::Duration::from_millis(500));

    for j in (self.min..self.max).rev() {
      println!("Pos: {}", j);
      self.servo_wrapper.servos.borrow_mut().set_pwm(self.pin, 1, j).unwrap();
      thread::sleep(time::Duration::from_millis(10));
    }

  }
}

#[derive(Debug)]
enum LegType { Front, Middle, Back }

#[derive(Debug)]
enum Side { Left, Right }

// Each leg for one side will have different motions asociated with a step... I.e. back legs will
// behave differently from front legs
struct Leg<'a> {
  side:     Side,
  leg_type: LegType,
  pelvis:   Joint<'a>,
  knee:     Joint<'a>,
  ankle:    Joint<'a>
}

impl<'a> Leg<'a> {
  fn new(side: Side, leg_type: LegType, pelvis: Joint<'a>, knee: Joint<'a>, ankle: Joint<'a>) -> Leg<'a> {
    Leg { side, leg_type, pelvis, knee, ankle }
  }

  fn reset(&mut self) {
    match self.side {
      Side::Left => {
        match self.leg_type {
          LegType::Front => {
            self.pelvis.mv(0);
            self.knee.mv(30);
            self.ankle.mv(120);
          }
          LegType::Middle => {
            self.pelvis.mv(90);
            self.knee.mv(30);
            self.ankle.mv(100);
          }
          LegType::Back => {
            self.pelvis.mv(180);
            self.knee.mv(30);
            self.ankle.mv(90);
          }
          _ => {
            println!("reset() Not implemented! {:?}", self.leg_type);
          }
        }
      }
      Side::Right => {
        match self.leg_type {
          LegType::Front => {
            self.pelvis.mv(180);
            self.knee.mv(140);
            self.ankle.mv(140);
          }
          LegType::Middle => {
            self.pelvis.mv(90);
            self.knee.mv(140);
            self.ankle.mv(140);
          }
          LegType::Back => {
            self.pelvis.mv(0);
            self.knee.mv(140);
            self.ankle.mv(140);
          }
          _ => {
            println!("reset() Not implemented! {:?}", self.leg_type);
          }
        }
      }
      _ => {
        println!("reset() Not implemented! {:?}", self.side);
      }
    }
  }

  fn lower(&mut self) {
    self.knee.mv(90);
    self.ankle.mv(140);
  }

  fn raise(&mut self) {
    self.knee.mv(120);
    self.ankle.mv(60);
  }

  fn forward(&mut self) {
    match self.leg_type {
      LegType::Front => {
        self.pelvis.mv(180);
      }
      LegType::Middle => {
        self.pelvis.mv(110);
      }
      LegType::Back => {
        self.pelvis.mv(90);
      }
      _ => {
        println!("{:?} forward() not implemented!", self.leg_type);
      }
    }
  }

  fn backward(&mut self) {
    match self.leg_type {
      LegType::Front => {
        self.pelvis.mv(90);
      }
      LegType::Middle => {
        self.pelvis.mv(80);
      }
      LegType::Back => {
        self.pelvis.mv(0);
      }
      _ => {
        println!("{:?} backward() not implemented!", self.leg_type);
      }
    }
  }

  fn step(&mut self, _dir: u16, speed: u64) {
    // match leg_type
    println!("Doing step!");

    println!("stepped!");
  }
}

struct Brain<'a> {
  fr: Leg<'a>,
  mr: Leg<'a>,
  br: Leg<'a>,
  fl: Leg<'a>,
  ml: Leg<'a>,
  bl: Leg<'a>,
}
impl<'a> Brain<'a> {
  fn new(left: &'a ServoWrapper, right: &'a ServoWrapper) -> Brain<'a> {
    println!("New brain!");

    Brain {
      br: Leg {
        side:     Side::Right,
        leg_type: LegType::Back,
        pelvis:   Joint::new(right, 0, 140, 580),
        knee:     Joint::new(right, 1, 120, 580),
        ankle:    Joint::new(right, 2, 110, 580),
      },
      mr: Leg {
        side:     Side::Right,
        leg_type: LegType::Middle,
        pelvis:   Joint::new(right, 3, 140, 580),
        knee:     Joint::new(right, 4, 120, 580),
        ankle:    Joint::new(right, 5, 110, 580),
      },
      fr: Leg {
        side:     Side::Right,
        leg_type: LegType::Front,
        pelvis:   Joint::new(right, 6, 120, 600),
        knee:     Joint::new(right, 7, 120, 580),
        ankle:    Joint::new(right, 8, 110, 580),
      },
      bl: Leg {
        side:     Side::Left,
        leg_type: LegType::Back,
        pelvis:   Joint::new(left, 0, 120, 600),
        knee:     Joint::new(left, 1, 120, 580),
        ankle:    Joint::new(left, 2, 110, 580),
      },
      ml: Leg {
        side:     Side::Left,
        leg_type: LegType::Middle,
        pelvis:   Joint::new(left, 3, 120, 620),
        knee:     Joint::new(left, 4, 120, 580),
        ankle:    Joint::new(left, 5, 110, 580),
      },
      fl: Leg {
        side:     Side::Left,
        leg_type: LegType::Front,
        pelvis:   Joint::new(left,  6, 140, 580),
        knee:     Joint::new(left,  7, 120, 580),
        ankle:    Joint::new(left,  8, 110, 580),
      }
    }
  }

  fn step_a(&mut self, dir: u16, speed: u64) {
    self.fl.reset();
    self.bl.reset();
    self.mr.reset();

    // thread::sleep(time::Duration::from_millis(speed * 2));

    // self.fl.backward();
    // self.bl.backward();
    // self.mr.backward();

    // thread::sleep(time::Duration::from_millis(speed));

    // self.fl.raise();
    // self.bl.raise();
    // self.mr.raise();

    // thread::sleep(time::Duration::from_millis(speed));

    // self.fl.forward();
    // self.bl.forward();
    // self.mr.forward();

    // thread::sleep(time::Duration::from_millis(speed));

    // self.fl.lower();
    // self.bl.lower();
    // self.mr.lower();
  }

  fn step_b(&mut self, dir: u16, speed: u64) {
    self.fr.reset();
    self.br.reset();
    self.ml.reset();

    // println!("reset");
    // thread::sleep(time::Duration::from_millis(speed * 2));

    // self.fr.backward();
    // self.ml.backward();
    // self.br.backward();

    // println!("backward");
    // thread::sleep(time::Duration::from_millis(speed));

    // self.fr.raise();
    // self.ml.raise();
    // self.br.raise();

    // println!("raise");
    // thread::sleep(time::Duration::from_millis(speed));

    // self.fr.forward();
    // self.ml.forward();
    // self.br.forward();

    // println!("forward");
    // thread::sleep(time::Duration::from_millis(speed));

    // self.fr.lower();
    // self.ml.lower();
    // self.br.lower();
    // println!("lower");
  }

  fn walk(&mut self, dir: u16, speed: u64) {
    self.step_b(dir, speed);
    thread::sleep(time::Duration::from_millis(speed));
    self.step_a(dir, speed);
  }
}

fn main() {
  // let left = ServoWrapper::new(0x40);
  // let right = ServoWrapper::new(0x41);
  // let mut brain = Brain::new(&left, &right);
  // brain.walk(0, 1000);
}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn test_walk() {
    let left = ServoWrapper::new(0x40);
    let right = ServoWrapper::new(0x41);
    let mut brain = Brain::new(&left, &right);
    brain.walk(0, 1000);
  }
}
