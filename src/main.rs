struct Servo {
    pin: u8,
    min: u64,
    max: u64
}

// Each leg for one side will have different motions asociated with a step... I.e. back legs will
// behave differently from front legs
struct Leg {
    pelvis: Servo,
    knee: Servo,
    ankle: Servo
}
impl Leg {
    fn new(&self, pelvis: Servo, knee: Servo, ankle: Servo) -> Leg {
        Leg { pelvis, knee, ankle }
    }
    fn step(&self, dir: u16) -> () {
        // Take one step in a direction. I guess we need to define NESW dirs and then do some
        // interpolation magic to get to the other 355 degrees...
    }
    fn rest(&self) -> () {
        // Go into resting position for this leg
    }
}

struct Brain {
    fl: Leg,
    fr: Leg,
    ml: Leg,
    mr: Leg,
    bl: Leg,
    br: Leg
}
impl Brain {
    fn walk(&self, speed: u8, dir: u16) -> () {

    }
}

fn main() {

}
