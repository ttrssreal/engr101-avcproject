use crate::config::SystemConfig;
use crate::pid::PID;
use crate::camera::Camera;
use crate::bcm2835;
use crate::robot::Robot;
use crate::gate::Gate;

#[derive(Debug)]
pub enum State {
    Stoped,
    Idle,
    GateClosed,
    GateOpen,
    FollowingCurve,
    Maze,
    Pillars,
    TrasitioningMaze,
}

pub struct System {
    camera: Camera,
    pid: PID,
    robot: Robot,
    gate: Gate,
    state: State,
    transition_ctr: u32,
}

impl System {
    pub fn new(conf: SystemConfig) -> Self {
        bcm2835::init(conf.debug as i32);
        if conf.debug { bcm2835::open_screen_stream(); }
        Self {
            camera: Camera::new(conf.camera),
            pid: PID::new(conf.pid),
            gate: Gate::new(conf.gate),
            robot: Robot::new(),
            state: State::Idle,
            transition_ctr: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), std::io::Error> {
        dbg!(&self.state);
        self.camera.update();
        self.state = match self.state {
            State::Stoped => {
                self.robot.stop();
                std::thread::sleep(std::time::Duration::from_millis(500));
                std::process::exit(0);
            },
            State::Idle => State::GateClosed,
            State::GateClosed => {
                self.gate.open_gate()?;
                State::GateOpen
            },
            State::GateOpen => State::FollowingCurve,
            State::FollowingCurve => {
                if self.camera.detect_marker() {
                    State::TrasitioningMaze
                } else {
                    if let Some(pos) = self.camera.get_line_pos() {
                        let turn = self.pid.output(pos);
                        self.robot.set_turn(turn);
                    }
                    State::FollowingCurve
                }
            },
            State::Maze => {
                if self.camera.detect_marker() {
                    State::Pillars
                } else {
                    // TODO: add maze logic
                    if let Some(pos) = self.camera.get_line_pos() {
                        let turn = self.pid.output(pos);
                        self.robot.set_turn(turn); 
                    }
                    State::Maze
                }
            },
            State::TrasitioningMaze => {
                self.transition_ctr += 1;
                if self.transition_ctr > 100 {
                    self.transition_ctr = 0;
                    State::Maze
                } else {
                    State::TrasitioningMaze
                }
            },
            State::Pillars => State::Stoped,
        };
        self.robot.update();
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state = State::Stoped;
        self.update().unwrap();
    }
}

impl Drop for System {
   fn drop(&mut self) {
       bcm2835::stoph();
   }
}
