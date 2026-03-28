use std::cmp::max;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{default, Commands, Component, Cuboid, Mesh, Name, ResMut, Transform, Vec3};
use bevy_rapier3d::prelude::*;
use crate::rat_mood::RatMoods;
use crate::rat_pilot::RatPilot;
use crate::resources::CResource;
//use crate::team::Team;
// TODO fix docs to fit better with Rust's documentation

#[derive(Component)]
pub struct Rat {
    pub health: CResource,
    pub morale: CResource,
    pub mood: RatMoods,
}

#[derive(Component)]
pub struct PlayerAllegiance;

#[derive(Component)]
pub struct EnemyAllegiance;

impl Rat {
    pub fn new(health: CResource, morale: CResource, mood: RatMoods) -> Self {
        Self {health, morale, mood}
    }
}

// pub fn spawn_rat(
//     name : String,
//     // mut team : Team,
//     transform : Transform,
//     color : Color,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // // against the advice of my lawyer, I have decided to not make them a gray box.
//     // // TODO use a loop to accomplish this multiplication
//     // let s = transform.scale;
//     // let rat_hull_points:&mut[Vec3] =  &mut[
//     //     Vec3::new( 0.000000*s.x, 0.000000*s.y,  2.414214*s.z, ),
//     //     Vec3::new( 0.000000*s.x, 0.000000*s.y, -1.414214*s.z, ),
//     //     Vec3::new( 1.051525*s.x, 0.000000*s.y, -0.362688*s.z, ),
//     //     Vec3::new( 1.173817*s.x, 0.000000*s.y,  0.410382*s.z, ),
//     //     Vec3::new(-1.173817*s.x, 0.000000*s.y,  0.410382*s.z, ),
//     //     Vec3::new(-1.051525*s.x, 0.000000*s.y, -0.362688*s.z, ),
//     //     Vec3::new( 0.000000*s.x, 1.051366*s.y, -0.362688*s.z, ),
//     //     Vec3::new( 0.000000*s.x, 1.312750*s.y,  0.436784*s.z, ),
//     // ];
//     commands
//         .spawn(RigidBody::Dynamic)
//         .insert(Collider::cuboid(
//             transform.scale.x/2.0,
//             transform.scale.y/2.0,
//             transform.scale.z/2.0,
//         )) // .
//         .insert(Restitution::coefficient(0.7))
//         .insert(transform)
//         .insert(GravityScale(9.81))
//         .insert(Name::new(name))
//         .insert(Mesh3d(meshes.add(Cuboid::new(
//             transform.scale.x,
//             transform.scale.y,
//             transform.scale.z,
//         ))))
//         .insert(MeshMaterial3d(materials.add(StandardMaterial {
//             base_color: color,
//             cull_mode: None,
//             ..default()
//         })));
// }