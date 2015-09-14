extern crate glutin;
// extern crate rand;
// use self::rand::Rng;

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
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Left)) => {
                self.nudge(0f32, -0.5f32);
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Right)) => {
                self.nudge(0f32, 0.5f32);
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Up)) => {
                self.nudge(0.5f32, 0f32);
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.nudge(-0.5f32, 0f32);
            },
            _ => {}
        }
    }
}