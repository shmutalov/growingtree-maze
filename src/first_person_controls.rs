use std::f32::consts::PI;

use glutin::VirtualKeyCode as Key;
use cgmath::{Point3, Vector3, Quaternion, InnerSpace, Rotation};
use mint;
use num;

use three::{Scene, Object};
use three::{Input, Button};

pub struct FirstPersonControls {
    object: Object,
    target: Point3<f32>,
    up: Option<mint::Vector3<f32>>,
    
    movement_speed: f32,
    look_speed: f32,

    look_vertical: bool,
    auto_forward: bool,
    // invertVertical: bool;
    active_look: bool,

    height_speed: bool,
    height_coef: f32,
    height_min: f32,
    height_max: f32,

    constrain_vertical: bool,
    vertical_min: f32,
    vertical_max: f32,

    auto_speed_factor: f32,

    mouse_x: f32,
    mouse_y: f32,

    lat: f32,
    lon: f32,
    phi: f32,
    theta: f32,

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
    pub fn new<P>(object: &Object, position: P, target: P) -> Self
        where P: Into<[f32; 3]>
    {
        let pf: [f32; 3] = position.into();
        let tf: [f32; 3] = target.into();

        let dir = (Point3::from(pf) - Point3::from(tf)).normalize();
        let up = Vector3::unit_y();
        let q = Quaternion::look_at(dir, up).invert();
        // TEMP
        let qv: [f32; 3] = q.v.into();
        let rot = mint::Quaternion {
            s: q.s,
            v: qv.into(),
        };
        let mut object = object.clone();
        object.set_transform(pf, rot, 1.0);

        FirstPersonControls {
            object: object,
            target: tf.into(),
            up: Some(mint::Vector3{x: up.x, y: up.y, z: up.z}),
            
            movement_speed: 1.0,
            look_speed: 0.005,

            look_vertical: true,
            auto_forward: false,
            // invertVertical: false;
            active_look: true,

            height_speed: false,
            height_coef: 1.0,
            height_min: 0.0,
            height_max: 1.0,

            constrain_vertical: false,
            vertical_min: 0.0,
            vertical_max: PI,

            auto_speed_factor: 0.0,

            mouse_x: 0.0,
            mouse_y: 0.0,

            lat: 0.0,
            lon: 0.0,
            phi: 0.0,
            theta: 0.0,

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

    pub fn update(&mut self, scene: &Scene, input: &Input) {
        self.handle_input(&input);

        if self.freeze {
            return;
        }

        let delta = input.time().get(&input);
        let mut pos = self.object.sync(&scene).world_transform.position;

        if self.height_speed {

            let y = num::clamp(pos.y, self.height_min, self.height_max);
            let height_delta = y - self.height_min;

            self.auto_speed_factor = delta * (height_delta * self.height_coef);

        } else {

            self.auto_speed_factor = 0.0;

        }

        let actual_move_speed = delta * self.movement_speed;

        let mut translate_x = 0.0;
        let mut translate_y = 0.0;
        let mut translate_z = 0.0;

        if self.move_forward || (self.auto_forward && !self.move_backward) {
            translate_z += -(actual_move_speed + self.auto_speed_factor);
        }
        if self.move_backward {
            translate_z += actual_move_speed;
        }

        if self.move_left {
            translate_x += -actual_move_speed;
        }
        if self.move_right {
            translate_x += actual_move_speed;
        }

        if self.move_up {
            translate_y += actual_move_speed;
        }
        if self.move_down {
            translate_y += -actual_move_speed;
        }

        // apply translation to object
        pos.x += translate_x;
        pos.y += translate_y;
        pos.z += translate_z;
        
        let mut actuallook_speed = delta * self.look_speed;

        if !self.active_look {
            actuallook_speed = 0.0;
        }

        let mut vertical_look_ratio = 1.0;

        if self.constrain_vertical {
            vertical_look_ratio = PI / (self.vertical_max - self.vertical_min);
        }

        self.lon += self.mouse_x * actuallook_speed;
        if self.look_vertical {
            self.lat -= self.mouse_y * actuallook_speed * vertical_look_ratio;
        }

        self.lat = -85.0_f32.max(85.0_f32.min(self.lat));
        self.phi = (90.0 - self.lat).to_radians();
        self.theta = self.lon.to_radians();

        if self.constrain_vertical {
            self.phi = self.map_linear(self.phi, 0.0, PI, self.vertical_min, self.vertical_max);
        }

        let mut target_pos = mint::Point3{x: self.target.x, y: self.target.y, z: self.target.z};

        target_pos.x = pos.x + 100.0 * self.phi.sin() * self.theta.cos();
        target_pos.y = pos.y + 100.0 * self.phi.cos();
        target_pos.z = pos.z + 100.0 * self.phi.sin() * self.theta.sin();

        self.object.set_position(pos);
        self.object.look_at(pos, target_pos, self.up);
    }

    fn map_linear (&self, x: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
		b1 + ( x - a1 ) * ( b2 - b1 ) / ( a2 - a1 )
	}
}
