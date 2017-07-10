use std::f32::consts::PI;

use glutin::VirtualKeyCode as Key;
use cgmath::{Point3, Vector3, Quaternion, InnerSpace, Rotation, One, Rad};
use cgmath::prelude::*;
use mint;
use num;

use three::{Scene, Object};
use three::{Input, Button, Timer};

pub struct FirstPersonControls {
    object: Object,
    position: Point3<f32>,

    movement_speed: f32,
    look_speed: f32,

    look_vertical: bool,
    auto_forward: bool,
    active_look: bool,

    auto_speed_factor: f32,

    _yaw: f32,
    _pitch: f32,
    mouse_x: f32,
    mouse_y: f32,
    last_mouse_x: f32,
    last_mouse_y: f32,

    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
    freeze: bool,

    forward_key: Button,
    backward_key: Button,
    strafe_left_key: Button,
    strafe_right_key: Button,
    upward_key: Button,
    downward_key: Button,
    freeze_key: Button,
}

impl FirstPersonControls {
    pub fn new<P>(object: &Object, position: P) -> Self
        where P: Into<[f32; 3]>
    {
        
        let pf: [f32; 3] = position.into();
        // let tf: [f32; 3] = target.into();

        // let dir = (Point3::from(pf) - Point3::from(tf)).normalize();
        // let up = Vector3::unit_y();
        // let q = Quaternion::look_at(dir, up).invert();
        // // TEMP
        // let qv: [f32; 3] = q.v.into();
        // let rot = mint::Quaternion {
        //     s: q.s,
        //     v: qv.into(),
        // };
        let mut object = object.clone();
        // object.set_transform(pf, rot, 1.0);

        FirstPersonControls {
            object: object,
            position: Point3::from(pf),

            movement_speed: 0.05,
            look_speed: 0.5,

            look_vertical: true,
            auto_forward: false,
            active_look: true,

            auto_speed_factor: 0.0,

            _yaw: 0.0,
            _pitch: 0.0,

            mouse_x: 0.0,
            mouse_y: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,

            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
            freeze: false,

            forward_key: Button::Key(Key::W),
            backward_key: Button::Key(Key::S),
            strafe_left_key: Button::Key(Key::A),
            strafe_right_key: Button::Key(Key::D),
            upward_key: Button::Key(Key::R),
            downward_key: Button::Key(Key::F),
            freeze_key: Button::Key(Key::Q),
        }
    }

    pub fn yaw(&mut self, amount: f32) {
        self._yaw += amount;
    }

    pub fn pitch(&mut self, amount: f32) {
        self._pitch += amount;
    }

    /// Moves the camera forward relative to its current rotation (yaw)
    pub fn walk_backward(&mut self, distance: f32) {
        self.position.x -= distance * self._yaw.to_radians().sin();
        self.position.z += distance * self._yaw.to_radians().cos();
    }

    // Moves the camera backward relative to its current rotation (yaw)
    pub fn walk_forward(&mut self, distance: f32) {
        self.position.x += distance * self._yaw.to_radians().sin();
        self.position.z -= distance * self._yaw.to_radians().cos();
    }

    // Strafes the camera left relitive to its current rotation (yaw)
    pub fn strafe_right(&mut self, distance: f32) {
        self.position.x -= distance * (self._yaw-90.0).to_radians().sin();
        self.position.z += distance * (self._yaw-90.0).to_radians().cos();
    }

    // Strafes the camera right relitive to its current rotation (yaw)
    pub fn strafe_left(&mut self, distance: f32) {
        self.position.x -= distance * (self._yaw+90.0).to_radians().sin();
        self.position.z += distance * (self._yaw+90.0).to_radians().cos();
    }

    // Translates and rotate the matrix so that it looks through the camera
    pub fn look_through(&mut self) {
        let pos: [f32; 3] = [self.position.x, self.position.y, self.position.z];
        
        let angle_x = Rad(self._pitch);
        let angle_y = Rad(self._yaw);

        let r: Quaternion<f32> = Quaternion::one() * Quaternion::from_angle_x(angle_x) * Quaternion::from_angle_y(angle_y);
        let rv: [f32; 3] = r.v.into();
        let rot = mint::Quaternion {
            s: r.s,
            v: rv.into(),
        };

        self.object.set_transform(pos, rot, 1.0);
    }

    fn handle_input(&mut self, input: &Input) {
        self.move_forward = self.forward_key.is_hit(&input);
        self.move_backward = self.backward_key.is_hit(&input);
        self.move_left = self.strafe_left_key.is_hit(&input);
        self.move_right = self.strafe_right_key.is_hit(&input);
        self.move_up = self.upward_key.is_hit(&input);
        self.move_down = self.downward_key.is_hit(&input);
        self.freeze = self.freeze_key.is_hit(&input);

        let mouse_pos = input.get_mouse_pos();
        self.mouse_x = mouse_pos.x;
        self.mouse_y = mouse_pos.y;
    }

    pub fn update(&mut self, input: &Input, timer: &Timer) {
        self.handle_input(&input);

        if self.freeze {
            return;
        }

        let delta = timer.get(&input);
        let mouse_dx = self.last_mouse_x - self.mouse_x;
        let mouse_dy = self.last_mouse_y - self.mouse_y;
        let speed = self.look_speed * delta;

        //controll camera yaw from x movement fromt the mouse
        self.yaw(mouse_dx * speed);

        //controll camera pitch from y movement fromt the mouse
        self.pitch(mouse_dy * speed);

        let actual_move_speed = delta * self.movement_speed;

        if self.move_forward {
            self.walk_forward(actual_move_speed);
        }

        if self.move_backward {
            self.walk_backward(actual_move_speed);
        }

        if self.move_left {
            self.strafe_left(actual_move_speed);
        }

        if self.move_right {
            self.strafe_right(actual_move_speed);
        }

        // if self.move_up {
        //     translate_y += actual_move_speed;
        // }
        // if self.move_down {
        //     translate_y += -actual_move_speed;
        // }

        self.look_through();

        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
    }

    fn map_linear(&self, x: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
        b1 + (x - a1) * (b2 - b1) / (a2 - a1)
    }
}
