use bevy::asset::Assets;
use bevy::color::Color;
use bevy::light::DirectionalLight;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{default, Commands, Meshable, Name, Plane3d, Res, ResMut, Resource, Transform};
use bevy_rapier3d::geometry::Collider;
use crate::camera::CameraSettings;

#[derive(Resource)]
pub struct WorldSettings {
    pub x_scale: f32,
    pub z_scale: f32,
    pub wall_height: f32,
    pub wall_thickness: f32,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            x_scale: 6.0,
            z_scale: 9.0,
            wall_height: 10.0,
            // walls are prohibitively thick to prevent any delta-t teleporting
            // when the RatPilot inevitably loses its mind. :)
            wall_thickness: 1.0, // also applies to the floor :)
        }
    }
}

pub fn setup_world_bounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
) {


    // build the world bounds
    // GROUND COLLIDER
    commands
        .spawn(Name::new("Ground"))
        .insert(Collider::cuboid( 100.0, world_settings.wall_thickness, 100.0))
        .insert(Transform::from_xyz(0.0, -world_settings.wall_thickness, 0.0))
        .insert(Mesh3d(meshes.add(Plane3d::default().mesh().size(world_settings.x_scale, world_settings.z_scale))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            cull_mode: None,
            ..default()
        })));
    // WALL COLLIDERS
    commands
        .spawn(Name::new("Rear Wall"))
        .insert(Collider::cuboid(world_settings.wall_thickness, world_settings.wall_height+world_settings.wall_thickness, world_settings.z_scale/2.0+world_settings.wall_thickness*2.0))
        .insert(Transform::from_xyz(-world_settings.x_scale/2.0-world_settings.wall_thickness, world_settings.wall_height-world_settings.wall_thickness, 0.0));
    commands
        .spawn(Name::new("Forward Wall"))
        .insert(Collider::cuboid(world_settings.wall_thickness, world_settings.wall_height+world_settings.wall_thickness, world_settings.z_scale/2.0+world_settings.wall_thickness*2.0))
        .insert(Transform::from_xyz(world_settings.x_scale/2.0+world_settings.wall_thickness, world_settings.wall_height-world_settings.wall_thickness, 0.0));
    commands
        .spawn(Name::new("Left Wall"))
        .insert(Collider::cuboid(world_settings.x_scale/2.0+world_settings.wall_thickness*2.0, world_settings.wall_height+world_settings.wall_thickness, world_settings.wall_thickness))
        .insert(Transform::from_xyz(0.0, world_settings.wall_height-world_settings.wall_thickness, -world_settings.z_scale/2.0-world_settings.wall_thickness));
    commands
        .spawn(Name::new("Right Wall"))
        .insert(Collider::cuboid(world_settings.x_scale/2.0+world_settings.wall_thickness*2.0, world_settings.wall_height+world_settings.wall_thickness, world_settings.wall_thickness))
        .insert(Transform::from_xyz(0.0, world_settings.wall_height-world_settings.wall_thickness, world_settings.z_scale/2.0+world_settings.wall_thickness));

}