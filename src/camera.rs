use itertools::Itertools;

use crate::{bcm2835, config::CameraConfig};

pub struct Camera {
    conf: CameraConfig,
}

impl Camera {
    pub fn new(conf: CameraConfig) -> Self {
        Self { conf }
    }

    pub fn update(&self) {
        bcm2835::take_picture();
    }

    pub fn show(&self) {
        bcm2835::update_screen();
    }

    pub fn get_line_pos(&self) -> Option<u32> {
        // average the 3 channels
        let calc_gs = |x| {
            (0..3).map(|ch| {
                bcm2835::get_pixel(self.conf.frame_height / 2, x, ch) as u32
            })
            .sum::<u32>() / 3
        };

        let (gs_min, gs_max) = (0..self.conf.frame_width)
            .map(calc_gs)
            .minmax()
            .into_option()?;

        let gs_thres = (gs_max + gs_min) / 2;

        // filter black pixels -> calc sum of positions and number of samples
        let (total, ns): (u32, u32) = (0..self.conf.frame_width)
            .filter_map(|x| Some(x).filter(|&x| calc_gs(x) < gs_thres))
            .fold((0, 0), |(total, ns), x: u32| {
                (total + x, ns + 1)
            });

        // average the positions of the black pixels
        total.checked_div(ns)
    }

    pub fn detect_marker(&self) -> bool {
        let scan_line_h = self.conf.frame_height / 2;

        // is red / (sum rgb) > 0.5?
        let is_red = |x| {
            let red = bcm2835::get_pixel(scan_line_h, x, 0) as f32;
            let tot = (0..3).map(|ch| {
                bcm2835::get_pixel(scan_line_h, x, ch) as u32
            }).sum::<u32>() as f32;
            red / tot > 0.5
        };

        // count red pixels along the scan line
        let red = (0..self.conf.frame_width)
            .map(is_red)
            .filter(|&x| x)
            .count();

        red > 120
    }
}
