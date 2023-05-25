use crate::config::SystemConfig;
use crate::pid::PID;
use crate::camera::Camera;
use crate::bcm2835;

pub struct System {
    camera: Camera,
    pid: PID,
}

impl System {
    pub fn new(conf: SystemConfig) -> Self {
        bcm2835::init(conf.debug_mode as i32);
        bcm2835::open_screen_stream();
        Self {
            camera: Camera::new(conf.camera),
            pid: PID::new(conf.pid),
        }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_pid(&self) -> &PID {
        &self.pid
    }
}

impl Drop for System {
   fn drop(&mut self) {
       bcm2835::stoph();
   }
}
