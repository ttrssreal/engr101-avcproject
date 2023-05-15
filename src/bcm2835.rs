#![allow(dead_code)]

// the library was built for c++ so has a shitty ABI and symbols :)
#[link(name = "e101")]
extern {
    pub fn _Z5stophv();
    pub fn _Z4initi(debug_level: i32) -> i32;
    // camera functions
    pub fn _Z12take_picturev() -> i32;
    pub fn _Z12save_picturePc(r#fn: &[i8; 5]) -> i32;
    pub fn _Z9get_pixeliii(row: i32, col: i32, color: i32) -> i8;
    pub fn _Z9set_pixeliiccc(row: i32, col: i32, red: i8, green: i8, blue: i8) -> i32;
    pub fn _Z24convert_camera_to_screenv();
    pub fn _Z18open_screen_streamv() -> i32;
    pub fn _Z19close_screen_streamv() -> i32;
    pub fn _Z13update_screenv() -> i32;
    pub fn _Z15display_pictureii(delay_sec: i32, delay_usec: i32) -> i32;
    // hardware control functions
    pub fn _Z10set_motorshh(num_mot: u8, pwm: u8) -> i32;
    pub fn _Z6sleep1i(msec: i32) -> i32;
    pub fn _Z11set_digitalhh(chan: u8, level: u8) -> i32;
    pub fn _Z12read_digitalh(chan: i32) -> i32;
    pub fn _Z11read_analogi(in_ch_adc: i32) -> i32;
    pub fn _Z17hardware_exchangev() -> i32;
    // networking functions
    pub fn _Z17connect_to_serverPci(server_addr: &[i8; 15],port: i32) -> i32;
    pub fn _Z14send_to_serverPc(message: &[i8; 24]) -> i32;
    pub fn _Z19receive_from_serverPc(message: &[i8; 24]) -> i32;
}

// rust FFI's

pub fn set_motors(nm: u8, sp: u8) -> i32 {
    unsafe {
        _Z10set_motorshh(nm, sp)
    }
}

pub fn take_picture() -> i32 {
    unsafe {
        _Z12take_picturev()
    }
}

pub fn stoph() {
    unsafe {
        _Z5stophv();
    };
}

pub fn init(debug_level: i32) -> i32 {
    unsafe {
        _Z4initi(debug_level)
    }
}

pub fn save_picture(filename: &[i8; 5]) -> i32 {
    unsafe {
        _Z12save_picturePc(filename)
    }
}

pub fn get_pixel(row: i32, col: i32, color: i32) -> i8 {
    unsafe {
        _Z9get_pixeliii(row, col, color)
    }
}

pub fn set_pixel(row: i32, col: i32, red: i8, green: i8, blue: i8) -> i32 {
    unsafe {
        _Z9set_pixeliiccc(row, col, red, green, blue)
    }
}

pub fn convert_camera_to_screen() {
    unsafe {
        _Z24convert_camera_to_screenv();
    }
}

pub fn open_screen_stream() -> i32 {
    unsafe {
        _Z18open_screen_streamv()
    }
}

pub fn close_screen_stream() -> i32 {
    unsafe {
        _Z19close_screen_streamv()
    }
}

pub fn update_screen() -> i32 {
    unsafe {
        _Z13update_screenv()
    }
}

pub fn display_picture(delay_sec: i32, delay_usec: i32) -> i32 {
    unsafe {
        _Z15display_pictureii(delay_sec, delay_usec)
    }
}

pub fn sleep1(msec: i32) -> i32 {
    unsafe {
        _Z6sleep1i(msec)
    }
}

pub fn set_digital(chan: u8, level: u8) -> i32 {
    unsafe {
        _Z11set_digitalhh(chan, level)
    }
}

pub fn read_digital(chan: i32) -> i32 {
    unsafe {
        _Z12read_digitalh(chan)
    }
}

pub fn read_analog(in_ch_adc: i32) -> i32 {
    unsafe {
        _Z11read_analogi(in_ch_adc)
    }
}

pub fn hardware_exchange() -> i32 {
    unsafe {
        _Z17hardware_exchangev()
    }
}

pub fn connect_to_server(server_addr: &[i8; 15], port: i32) -> i32 {
    unsafe {
        _Z17connect_to_serverPci(server_addr, port)
    }
}

pub fn send_to_server(message: &[i8; 24]) -> i32 {
    unsafe {
        _Z14send_to_serverPc(message)
    }
}

pub fn receive_from_server(message: &mut [i8; 24]) -> i32 {
    unsafe {
        _Z19receive_from_serverPc(message)
    }
}
