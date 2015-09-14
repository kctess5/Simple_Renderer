#![allow(dead_code)]
extern crate glutin;
extern crate vecmath;
use self::glutin::VirtualKeyCode;
use self::vecmath::{
    Vector3,
    vec3_cross,
    vec3_normalized,
    vec3_add,
    vec3_scale,
    vec3_dot,
    Matrix4,
    vec3_normalized_sub
};

use std::f32;

/// First person camera settings.
pub struct FirstPersonSettings {
    /// Which button to press to move forward.
    pub move_forward_button: VirtualKeyCode,
    /// Which button to press to move backward.
    pub move_backward_button: VirtualKeyCode,
    /// Which button to press to strafe left.
    pub strafe_left_button: VirtualKeyCode,
    /// Which button to press to strafe right.
    pub strafe_right_button: VirtualKeyCode,
    /// Which button to press to fly up.
    pub fly_up_button: VirtualKeyCode,
    /// Which button to press to fly down.
    pub fly_down_button: VirtualKeyCode,
    /// Which button to press to move faster.
    pub move_faster_button: VirtualKeyCode,
    /// The horizontal movement speed. units per second.
    pub speed_horizontal: f32,
    /// The vertical movement speed. units per second.
    pub speed_vertical: f32,
}

impl FirstPersonSettings {
    /// Creates new first person camera settings with wasd defaults.
    pub fn keyboard_wasd() -> FirstPersonSettings {
        FirstPersonSettings {
            move_forward_button: VirtualKeyCode::Space,
            move_backward_button: VirtualKeyCode::LShift,
            strafe_left_button: VirtualKeyCode::A,
            strafe_right_button: VirtualKeyCode::D,
            fly_up_button: VirtualKeyCode::W,
            fly_down_button: VirtualKeyCode::S,
            move_faster_button: VirtualKeyCode::LControl,
            speed_horizontal: 0f32,
            speed_vertical: 0f32,
        }
    }
}

#[allow(dead_code)]
pub struct CameraState {
    aspect_ratio: f32,
    position: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub forward: Vector3<f32>,
    keys: Vec<VirtualKeyCode>,
    mouse: (i32, i32),
    dx: i32,
    dy: i32,
    velocity: f32,
    settings: FirstPersonSettings,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: [0.0, 0.0, 6.0],
            right:   [1f32, 0f32, 0f32],
            up:      [0f32, 1f32, 0f32],
            forward: [0f32, 0f32, 1f32],
            keys: Vec::new(),
            mouse: (0i32, 0i32),
            dx: 0i32,
            dy: 0i32,
            yaw: 0f32,
            pitch: 0f32,
            velocity: 0.05f32,
            settings: FirstPersonSettings::keyboard_wasd(),
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;
        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0,              0.0              , 0.0],
            [         0.0         , f ,               0.0              , 0.0],
            [         0.0         , 0.0,  (zfar+znear)/(znear-zfar)    ,-1.0],
            [         0.0         , 0.0,  (2.0*zfar*znear)/(znear-zfar), 0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = vec3_normalized(self.forward);
        let up = self.up;
        let _0: f32 = 0f32;
        let _1: f32 = 1f32;

        let s = [f[1] * up[2] - f[2] * up[1],
                 f[2] * up[0] - f[0] * up[2],
                 f[0] * up[1] - f[1] * up[0]];

        let s_norm = vec3_normalized(s);

        let u = [s_norm[1] * f[2] - s_norm[2] * f[1],
                 s_norm[2] * f[0] - s_norm[0] * f[2],
                 s_norm[0] * f[1] - s_norm[1] * f[0]];

        let p = [-self.position[0] * s[0] - self.position[1] * s[1] - self.position[2] * s[2],
                 -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
                 -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s[0], u[0], -f[0], _0],
            [s[1], u[1], -f[1], _0],
            [s[2], u[2], -f[2], _0],
            [p[0], p[1],  p[2], _1],
        ]
    }

    fn update_right(&mut self) {
        self.right = vec3_cross(self.up, self.forward);
    }

    /// Computes an orthogonal matrix for the camera.
    ///
    /// This matrix can be used to transform coordinates to the screen.
    pub fn orthogonal(&self) -> Matrix4<f32> {
        let p = self.position;
        let r = self.right;
        let u = self.up;
        let f = self.forward;
        let _0 = 0f32;
        [
            [r[0], u[0], f[0], _0],
            [r[1], u[1], f[1], _0],
            [r[2], u[2], f[2], _0],
            [-vec3_dot(r, p), -vec3_dot(u, p), -vec3_dot(f, p), 1f32]
        ]
    }

    /// Orients the camera to look at a point.
    pub fn look_at(&mut self, point: Vector3<f32>) {
        self.forward = vec3_normalized_sub(self.position, point);
        self.update_right();
    }

    /// Sets yaw and pitch angle of camera in radians.
    pub fn set_yaw_pitch(&mut self) {
        let (y_s, y_c, p_s, p_c) = (self.yaw.sin(), self.yaw.cos(), self.pitch.sin(), self.pitch.cos());
        self.forward = [y_s * p_c, p_s, y_c * p_c];
        self.up = [y_s * -p_s, p_c, y_c * -p_s];
        self.update_right();
    }

    pub fn update(&mut self) {
        self.update_dir();
        self.update_pos();
        // println!("{:?} {:?}", self.position, self.forward);
    }

    fn update_dir(&mut self) {
        let pi: f32 = f32::consts::PI;
        let _0 = 0f32;
        let _1 = 1f32;
        let _2 =  _1 + _1;
        let _3 = _2 + _1;
        let _4 = _3 + _1;
        let _360 = 360f32;
        let dy: f32 = self.dy as f32;
        let dx: f32 = self.dx as f32;

        if !(self.keys.iter().position(|&r| r == VirtualKeyCode::Yen) == None) {
            self.yaw = (self.yaw - dx / _360 * pi / _4) % (_2 * pi);
            self.pitch = self.pitch + dy / _360 * pi / _4;
            self.pitch = self.pitch.min(pi / _2).max(-pi / _2);
        }

        self.set_yaw_pitch();
    }

    fn update_pos(&mut self) {
        if self.keys.contains(&self.settings.move_forward_button) {
           let dist = -self.velocity;
           let displacement = vec3_scale(self.forward, dist);
           self.position = vec3_add(displacement, self.position); 
        }

        if self.keys.contains(&self.settings.move_backward_button) {
           let dist = self.velocity;
           let displacement = vec3_scale(self.forward, dist);
           self.position = vec3_add(displacement, self.position); 
        }

        if self.keys.contains(&self.settings.strafe_left_button) {
           let dist = self.velocity;
           let displacement = vec3_scale(self.right, dist);
           self.position = vec3_add(displacement, self.position); 
        }

        if self.keys.contains(&self.settings.strafe_right_button) {
           let dist = -self.velocity;
           let displacement = vec3_scale(self.right, dist);
           self.position = vec3_add(displacement, self.position); 
        }

        if self.keys.contains(&self.settings.fly_up_button) {
           let dist = self.velocity;
           let displacement = vec3_scale(self.up, dist);
           self.position = vec3_add(displacement, self.position); 
        }

        if self.keys.contains(&self.settings.fly_down_button) {
           let dist = -self.velocity;
           let displacement = vec3_scale(self.up, dist);
           self.position = vec3_add(displacement, self.position); 
        }
    }

    pub fn process_input(&mut self, e: &glutin::Event) {
        use self::glutin::ElementState::{Released, Pressed};
        use self::glutin::Event::{KeyboardInput, MouseMoved, MouseInput};

        match e {
            &KeyboardInput(Pressed, _, Some(ref button)) => {
                if !self.keys.contains(button) {
                    self.keys.push(*button);
                }
            },
            &KeyboardInput(Released, _, Some(ref button)) => {
                if self.keys.contains(button) {
                    let i = self.keys.iter().position(|&r| r == *button).unwrap();
                    self.keys.remove(i);
                }
            },
            &MouseInput(Pressed, _) => {
                if !self.keys.contains(&VirtualKeyCode::Yen) {
                    self.keys.push(VirtualKeyCode::Yen);
                }
            },
            &MouseInput(Released, _) => {
                let i = self.keys.iter().position(|&r| r == VirtualKeyCode::Yen).unwrap();
                self.keys.remove(i);
            },
            &MouseMoved((ref x, ref y)) => {
                self.dx = x - self.mouse.0;
                self.dy = self.mouse.1 - y;
                self.mouse = (*x, *y);
            },
            _ => {}
        }
        // println!("{:?}", self.keys);
    }
}