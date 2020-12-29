use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseScrollUnit::{Line, Pixel};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const LINE_TO_PIXEL_RATIO: f32 = 0.1;

#[derive(Default)]
struct State {
    motion: EventReader<MouseMotion>,
    scroll: EventReader<MouseWheel>,
}

pub struct OrbitCamera {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: 5.0,
            center: Vec3::zero(),
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

impl OrbitCamera {
    pub fn new(dist: f32, center: Vec3) -> OrbitCamera {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: dist,
            center,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

pub struct OrbitCameraPlugin;
impl OrbitCameraPlugin {
    fn mouse_motion_system(
        time: Res<Time>,
        mut state: ResMut<State>,
        mouse_motion_events: Res<Events<MouseMotion>>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut delta = Vec2::zero();
        for event in state.motion.iter(&mouse_motion_events) {
            delta += event.delta;
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            if mouse_button_input.pressed(MouseButton::Left) {
                camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();

                camera.y = camera.y.clamp(0.01, 3.13);

                let rot = Quat::from_axis_angle(Vec3::unit_y(), camera.x)
                    * Quat::from_axis_angle(-Vec3::unit_x(), camera.y);
                transform.translation =
                    (rot * Vec3::new(0.0, 1.0, 0.0)) * camera.distance + camera.center;
                transform.look_at(camera.center, Vec3::unit_y());
            }
        }
    }

    fn zoom_system(
        mut state: ResMut<State>,
        mouse_wheel_events: Res<Events<MouseWheel>>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut total = 0.0;
        for event in state.scroll.iter(&mouse_wheel_events) {
            total += event.y
                * match event.unit {
                    Line => 1.0,
                    Pixel => LINE_TO_PIXEL_RATIO,
                };
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            camera.distance *= camera.zoom_sensitivity.powf(total);
            let translation = &mut transform.translation;
            *translation =
                (*translation - camera.center).normalize() * camera.distance + camera.center;
        }
    }
}
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_system(Self::mouse_motion_system.system())
            .add_system(Self::zoom_system.system());
    }
}
