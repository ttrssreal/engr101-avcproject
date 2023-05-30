use crate::config::PidConfig;

pub struct PID {
    params: PidConfig,
    target: u32,
}

impl PID {
    pub fn new(config: PidConfig) -> Self {
        Self {
            params: config,
            target: 320/2, // center of screen
        }
    }
    
    pub fn set_target(&mut self, target: u32) {
        self.target = target;
    }

    pub fn output(&self, line_pos: u32) -> i32 {
        let co_eff = &self.params;
        let e = (line_pos as i32 - self.target as i32) as f32;
        (e * co_eff.kp) as i32
    }
}
