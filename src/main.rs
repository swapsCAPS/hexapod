use std::{thread, time};
extern crate pwm_pca9685;

use pwm_pca9685::{Channel, Pca9685, SlaveAddr};

struct Joint {
  pin: u8,
  min: u8,
  max: u8,
  pos: u8,
}
impl Joint {
  fn new(pin: u8, min: u8, max: u8) -> Joint {
    let pos = max - min / 2;
    Joint { pin, min, max, pos }
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
struct Leg {
  leg_type: LegType,
  pelvis: Joint,
  knee: Joint,
  ankle: Joint
}

impl Leg {
  fn new(leg_type: LegType, mut pelvis: Joint, mut knee: Joint, mut ankle: Joint) -> Leg {
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

struct Brain {
  fl: Leg,
  // fr: Leg,
  // ml: Leg,
  // mr: Leg,
  // bl: Leg,
  // br: Leg
}
impl Brain {
  fn new() -> Brain {
    println!("New brain!");

    let fl = Leg::new(
      LegType::Front,
      Joint::new(0, 0, 180),
      Joint::new(1, 0, 180),
      Joint::new(2, 0, 180),
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
    let mut brain = Brain::new();
    brain.walk(0, 1000);

  }
}
