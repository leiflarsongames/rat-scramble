use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::geometry::Collider;
use crate::world::WorldSettings;


#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct CameraSettings {
    pub force:f32,
    pub home_position: Vec3,
    pub look_offset: Vec3,
    pub spring_constant: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let initial_home_position = Vec3::new(-1.0, 12.0, 0.0);
        let looking_at = Vec3::ZERO;
        Self {
            force : 0.10,
            home_position: initial_home_position,
            look_offset: looking_at - initial_home_position,
            spring_constant : 0.01,
        }

    }
}

pub fn setup_camera (
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
) {
    // TODO does the light do anything?
    // Lighting
    let mut light:DirectionalLight = DirectionalLight::default();
    light.illuminance = 1500.0;
    light.color = Color::WHITE;
    light.shadows_enabled = false;
    // See https://docs.rs/bevy/latest/bevy/prelude/struct.DirectionalLight.html
    // 150 is approx. illuminance of a subway platform
    commands.spawn((
        Name::new("Directional Light"),
        Transform::default().looking_to(Vec3::new(-0.5, -0.9, -0.1), Vec3::ZERO),
        light,
    ));



    // Camera
    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(
            camera_settings.home_position.x,
            camera_settings.home_position.y,
            camera_settings.home_position.z,
        ).looking_at(
            Vec3::ZERO,
            Vec3::Y
        ),
    ));
}


pub fn get_look_location(
    camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>, // TODO make sure this resource is available!
) -> Vec3
{
    return camera.translation + camera_settings.look_offset;
}

// /// Intended for use in Update... so each tick.
// pub fn move_camera(
//     mut camera: Single<&mut Transform, With<Camera>>,
//     camera_settings: Res<CameraSettings>, // TODO make sure this resource is available!
//     mouse_motion: Res<AccumulatedMouseMotion>,
//     // delta_time: Res<Time>,
//
// ) {
//     // TODO reimplement spring force!
//     // calculate forces
//     let camera_force:Vec2 = mouse_motion.delta * camera_settings.force;
//     let spring_force:Vec2 = Vec2::new(
//         camera.translation.x - camera_settings.home_position.x,
//         camera.translation.y - camera_settings.home_position.y
//     ) * camera_settings.spring_constant;
//
//     // apply the force to the camera!
//     camera.translation.x += -camera_force.y - spring_force.x;
//     camera.translation.z += -camera_force.x - spring_force.y;
//     // println!("camera position: {}", camera.translation);
//     // camera.translation.x = force_sum.x + camera_settings.home_position.x;
//     // camera.translation.z = force_sum.y + camera_settings.home_position.z;
//
//     // println!("looking @ {}", get_look_location(camera, camera_settings));
//     // println!("camera force: {}", camera_force);
//
//     // println!("spring force: {}", spring_force);
// }