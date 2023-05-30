use crate::bcm2835;

struct Motor {
    id: u8,
    speed: u8
}

pub struct Robot {
    motor_spd_changed: bool,
    left_motor: Motor,
    right_motor: Motor,
}

impl Motor {
    pub fn new(id: u8, init_speed: u8) -> Self {
        let mut motor = Self { id, speed: init_speed };
        motor.set_speed(init_speed);
        motor
    }
    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
        bcm2835::set_motors(self.id, speed);
    }
}

impl Robot {
    pub fn new() -> Self {
        Self {
            motor_spd_changed: false,
            left_motor: Motor::new(5, 56),
            right_motor: Motor::new(1, 40),
        }
    }

    pub fn update(&mut self) {
        if self.motor_spd_changed {
            bcm2835::hardware_exchange();
            self.motor_spd_changed = false;
        }
    }

    pub fn stop(&mut self) {
        self.left_motor.set_speed(48);
        self.right_motor.set_speed(48);
        self.motor_spd_changed = true;
        self.update();
    }

    // +'ve -> clockwise
    // -'ve -> anti-clockwise
    pub fn set_turn(&mut self, turn: i32) {
        let turn_clamped = turn.clamp(-10, 9);
        self.left_motor.set_speed((56 + turn_clamped) as u8);
        self.right_motor.set_speed((40 + turn_clamped) as u8);
        self.motor_spd_changed = true;
    }
}
