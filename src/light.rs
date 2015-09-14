extern crate glutin;

use self::glutin::Event::KeyboardInput;
use self::glutin::ElementState::Pressed;
use self::glutin::VirtualKeyCode;

pub struct Light {
    data: [f32; 3]
}

impl Light {
	pub fn new() -> Light {
		Light { data: [ 1.0, 1.0, 1.0f32] }
	}
	pub fn serialize(&mut self) -> [f32; 3] {
		self.data
	}
	pub fn nudge(&mut self, l: f32, u: f32) {
		self.data[0] += u;
		self.data[1] += l;
	}
	pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Left)) => {
                self.nudge(0f32, -0.5f32);
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Right)) => {
                self.nudge(0f32, 0.5f32);
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Up)) => {
                self.nudge(0.5f32, 0f32);
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Down)) => {
                self.nudge(-0.5f32, 0f32);
            },
            _ => {}
        }
    }
}