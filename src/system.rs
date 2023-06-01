use crate::config::SystemConfig;
use crate::pid::PID;
use crate::camera::Camera;
use crate::bcm2835;
use crate::robot::Robot;
use crate::gate::Gate;

enum Turn {
    Left,
    Right,
}

use Turn::*;
static TURN_LOOKUP: [Turn; 7] = [ Right, Right, Left, Left, Right, Left, Right ];

#[derive(Debug, PartialEq)]
pub enum State {
    Stopped,
    Idle,
    GateClosed,
    GateOpen,
    FollowingCurve,
    Maze,
    Pillars,
    TrasitioningMaze,
    FoundIntersection,
}

pub struct System {
    camera: Camera,
    pid: PID,
    robot: Robot,
    gate: Gate,
    state: State,
    transition_timer: u32,
    intersection_timer: u32,
    maze_turn_idx: usize,
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
            transition_timer: 0,
            intersection_timer: 0,
            maze_turn_idx: 0,
        }
    }

    pub fn update(&mut self) -> Result<bool, std::io::Error> {
        dbg!(&self.state);
        self.camera.update();
        self.state = match self.state {
            // Stopped is a terminal state so return and finish
            State::Stopped => {
                self.robot.stop();
                State::Stopped
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
                    if let Some(pos) = self.camera.get_line_pos() {
                        let turn = self.pid.output(pos);
                        self.robot.set_turn(turn); 
                    }
                    if self.camera.detect_intersection() {
                        State::FoundIntersection
                    } else {
                        State::Maze
                    }
                }
            },
            State::FoundIntersection => {
                assert!(self.maze_turn_idx < TURN_LOOKUP.len());
                // Hardcoded 25 iterations of turning generalises
                // to all turns in the maze.
                if self.intersection_timer > 25 {
                    self.maze_turn_idx += 1;
                    self.intersection_timer = 0;
                    State::Maze
                } else {
                    self.intersection_timer += 1;
                    let offset = match TURN_LOOKUP[self.maze_turn_idx] {
                        Left => -10,
                        Right => 9,
                    };
                    self.robot.set_turn(offset);
                    State::FoundIntersection
                }
            },
            // This state is used to skip over the section markers
            // i.e avoid multiple detections etc
            State::TrasitioningMaze => {
                self.transition_timer += 1;
                if self.transition_timer > 50 {
                    State::Maze
                } else {
                    self.robot.set_turn(0);
                    State::TrasitioningMaze
                }
            },
            State::Pillars => State::Stopped,
        };
        self.robot.update();
        Ok(self.state == State::Stopped) // exit if stopped
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped;
        self.update().unwrap();
    }
}

impl Drop for System {
   fn drop(&mut self) {
       bcm2835::stoph();
   }
}
