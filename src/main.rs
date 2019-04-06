use std::{thread, time};

struct Joint {
  pin: u8,
  min: u64,
  max: u64
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
  fn new(leg_type: LegType, pelvis: Joint, knee: Joint, ankle: Joint) -> Leg {
    Leg { leg_type, pelvis, knee, ankle }
  }

  fn step(&self, dir: u16) -> () {
    // match leg_type
    println!("Step!");
    match self.leg_type {
      LegType::Front => {
        println!("Moving front leg");
      }
      _ => {
        println!("Not implemented!");
      }
    }

  }

  fn rest(&self) -> () {
    // Go into resting position for this leg
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
      Joint { pin: 0, min: 0, max: 180 },
      Joint { pin: 1, min: 0, max: 180 },
      Joint { pin: 2, min: 0, max: 180 },
    );

    Brain {
      fl: fl
    }
  }

  fn walk(&self, speed: u64, dir: u16) -> () {
    self.fl.step(dir);
    thread::sleep(time::Duration::from_millis(speed));
    self.fl.step(dir);

  }
}

fn main() {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_walk() {
    let brain = Brain::new();
    brain.walk(1000, 0);

  }
}
