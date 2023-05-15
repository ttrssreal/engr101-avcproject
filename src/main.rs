mod bcm2835;

// test functions all work on system

fn main() {
    println!("ENG101 - AVC Project.");
    bcm2835::init(0);
    bcm2835::open_screen_stream();
    for _ in 0..100 {
        bcm2835::take_picture();
        bcm2835::update_screen();
    }
    bcm2835::hardware_exchange();
    bcm2835::sleep1(1000);
    bcm2835::stoph();
}