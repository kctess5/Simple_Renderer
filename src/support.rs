extern crate clock_ticks;

use std::thread;

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(time_stamp: u64, mut callback: F) where F: FnMut(f32, u32) -> Action {
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut i = 0;
    loop {
        let now = clock_ticks::precise_time_ns();
        let dt = now - previous_clock;
        
        match callback((dt as f32) / 1000000000f32, i) {
            Action::Stop => break,
            Action::Continue => ()
        };

        i += 1;
        accumulator += dt;
        previous_clock = now;

        while accumulator >= time_stamp {
            accumulator -= time_stamp;
        }

        thread::sleep_ms(((time_stamp - accumulator) / 1000000) as u32);
    }
}
