use cgmath::SquareMatrix;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::State;
pub struct Camera {
    pos: cgmath::Point3<f32>,
    // target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    _fovy: f32,
    znear: f32,
    zfar: f32,

    view_proj: [[f32; 4]; 4],

    speed: f32,
    sensitivity: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
}
impl Camera {
    pub fn default(aspect: f32) -> Self {
        Self {
            pos: (-5.0, 0.0, 0.0).into(),
            pitch: 0.0,
            yaw: 0.0,
            up: cgmath::Vector3::unit_y(), // Set the UP direction
            aspect,
            _fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,

            view_proj: cgmath::Matrix4::identity().into(),

            speed: 0.1,
            sensitivity: 0.005,
            is_backward_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        use cgmath::{InnerSpace, Matrix4, Rad, Vector3};

        let (yaw, pitch) = (Rad(self.yaw), Rad(self.pitch));

        // calculate forward direction from yaw/pitch
        let direction = Vector3::new(
            yaw.0.cos() * pitch.0.cos(),
            pitch.0.sin(),
            yaw.0.sin() * pitch.0.cos(),
        )
        .normalize();

        let target = self.pos + direction;
        let view = Matrix4::look_at_rh(self.pos, target, self.up);

        let proj = cgmath::perspective(
            Rad(std::f32::consts::FRAC_PI_4),
            self.aspect,
            self.znear,
            self.zfar,
        );

        return proj * view;
    }
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    KeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    KeyCode::ControlLeft => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn update_camera(&mut self) {
        use cgmath::{InnerSpace, Rad, Vector3, Zero};

        let yaw = Rad(self.yaw);
        let pitch = Rad(self.pitch);

        let forward = Vector3::new(
            yaw.0.cos() * pitch.0.cos(),
            pitch.0.sin(),
            yaw.0.sin() * pitch.0.cos(),
        )
        .normalize();

        let right = forward.cross(self.up).normalize();
        let up = self.up;

        let mut movement = Vector3::zero();

        if self.is_forward_pressed {
            movement += forward;
        }
        if self.is_backward_pressed {
            movement -= forward;
        }
        if self.is_right_pressed {
            movement += right;
        }
        if self.is_left_pressed {
            movement -= right;
        }
        if self.is_up_pressed {
            movement += up;
        }
        if self.is_down_pressed {
            movement -= up;
        }

        if movement.magnitude2() > 0.0 {
            self.pos += movement.normalize() * self.speed;
        }

        self.update_view_proj();
    }
    pub fn process_mouse_motion(&mut self, dx: f64, dy: f64) {
        self.yaw += dx as f32 * self.sensitivity;
        self.pitch -= dy as f32 * self.sensitivity;

        const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
        const MIN_PITCH: f32 = -MAX_PITCH;

        self.pitch = self.pitch.clamp(MIN_PITCH, MAX_PITCH);
    }
    fn update_view_proj(&mut self) {
        self.view_proj = self.build_view_projection_matrix().into();
    }

    pub fn view_proj(&self) -> [[f32; 4]; 4] {
        self.view_proj
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update_view_proj();
    }
}
