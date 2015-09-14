extern crate glutin;
extern crate rand;

use self::glutin::Event::KeyboardInput;
use self::glutin::ElementState::Pressed;
use self::glutin::VirtualKeyCode;

pub struct Color {
    data: [f32; 3]
}

impl Color {
	pub fn new() -> Color {
		let mut c = Color { data: [ 1.0, 1.0, 1.0f32] };
		c.randomize();
		c
	}
	pub fn serialize(&mut self) -> [f32; 3] {
		self.data
	}
	pub fn randomize(&mut self) {
		self.data = [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()];
	}
	pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::C)) => {
                self.randomize();
            },
            _ => {}
        }
    }
}