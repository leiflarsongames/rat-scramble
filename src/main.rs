
mod camera;
mod pid;
mod rat_pilot;
mod resources;
mod utils;
// mod rat_mood_OLD;
// mod rat_mood_NOT_QUITE_AS_OLD_BUT_STILL_OLD;
mod rat;
mod rat_mood;
mod world;
// mod world;

use std::process::Termination;
use std::thread;
use std::time::Duration;
use bevy::{
    app::{AppExit},
    prelude::*,
};
use bevy::input::keyboard::KeyboardInput;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use crate::camera::CameraSettings;
use crate::pid::Pid;
use crate::rat::{EnemyAllegiance, PlayerAllegiance, Rat};
use crate::rat_mood::{RatMoodOperations, RatMoods};
use crate::rat_pilot::RatPilot;
use crate::resources::CResource;
// use crate::rat::spawn_rat;
use crate::world::WorldSettings;

const CURSOR_SPEED:f32 = 8.0;
const GRAVITY:f32 = 4.0;
const CONTROL_FORCE_FACTOR:f32 = 0.2;
const ABSCOND_FORCE_FACTOR:f32 = 0.01;

fn main() {
    // GAME META VARIABLES
    // TODO pull as much of this info as possible from Cargo.toml!
    let _game_name = env!("CARGO_PKG_NAME");
    let _game_vers = concat!("v", env!("CARGO_PKG_VERSION"));
    let _game_status = "PRERELEASE";
    let _maintainer_contact = "leiflarsongames@gmail.com or @neyewo on Discord";

    println!("{} {} {}",
        _game_name,
        _game_vers,
        _game_status,
    );

    // run game
    let app_exit:AppExit = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<CameraSettings>() // make CameraSettings resource available
        .init_resource::<WorldSettings>()
        .add_systems(Startup, (
            crate::world::setup_world_bounds,
            crate::camera::setup_camera,
            setup_cursors,
            setup_rats,
            write_instructions_to_screen,
        ))
        .add_systems(Update, (
            execute_rat_commands,
            walk_cursors,
            iterate_melee_damage,
        ))

        // TODO tell user game is now running!
        .run();

    if app_exit.is_success() {
        println!("Thank you for playing {} {} {}",
                 _game_name,
                 _game_vers,
                 _game_status,
        );
    }
    else {
        println!("{} {} {} exited with status {}.\n\
            Please report this error to {} in as much detail as possible.",
        _game_name,
        _game_vers,
        _game_status,
        "TODO", // TODO write out the exit status, and save a crash log with a stack trace!
        _maintainer_contact);
    }
    thread::sleep(Duration::from_secs(5)); // leave console up for 5 sec.
    app_exit.report();
}

// // applies a cubic slowing that kicks in over 10m/s!
// fn slowdown(
//     time: Res<Time>,
//     q_all_rats: Query<(&RigidBody, &mut ExternalForce)>,
// ) {
//     for rat in q_all_rats.iter() {
//         if rat.0.
//     }
// }

fn execute_rat_commands(
    mut commands: Commands,
    time: Res<Time>,
    q_player_rats: Query<(&mut RatPilot, &Transform, &mut ExternalForce, &Rat), (With<Rat>, With<PlayerAllegiance>, Without<EnemyAllegiance>)>,
    q_enemy_rats: Query<(&mut RatPilot, &Transform, &mut ExternalForce, &Rat), (With<Rat>, Without<PlayerAllegiance>, With<EnemyAllegiance>)>,
    q_player_cursor: Single<&Transform, With<PlayerCursor>>,
    q_enemy_cursor: Single<&Transform, (With<EnemyCursor>, Without<PlayerCursor>)>,

) {
    // find cursor positions
    let p_curs_pos = q_player_cursor.translation;
    let e_curs_pos = q_enemy_cursor.translation;
    // issue commands to the appropriate teams
    // PLAYER COMMANDS
    // let mut doing_debug = true;
    for mut p_rat in q_player_rats {
        p_rat.2.force = match p_rat.3.mood {
            // run towards team's cursor
            RatMoods::FRESH =>
                p_rat.0.update(
                    p_curs_pos,
                    p_rat.1.translation,
                    time.delta_secs(),
                ),
            RatMoods::SPOOKED =>
                // run away from other team's cursor
                (p_rat.1.translation - e_curs_pos).normalize_or_zero()
                    *ABSCOND_FORCE_FACTOR *
                    Vec3::new(1.0, 0.0, 1.0), // eliminate Y-axis
            RatMoods::DOWN =>
                // do nothing
                Vec3::ZERO,
        }
    }
    // ENEMY COMMANDS
    for mut e_rat in q_enemy_rats {
        // apply force from command
        e_rat.2.force = match e_rat.3.mood {
            // run towards team's cursor
            RatMoods::FRESH =>
                e_rat.0.update(
                    e_curs_pos,
                    e_rat.1.translation,
                    time.delta_secs(),
                ),
            RatMoods::SPOOKED =>
            // run away from other team's cursor
                (e_rat.1.translation - p_curs_pos).normalize_or_zero()
                    *ABSCOND_FORCE_FACTOR *
                    Vec3::new(1.0, 0.0, 1.0), // eliminate Y-axis
            RatMoods::DOWN =>
            // do nothing
                Vec3::ZERO,
        }
    }
    // println!("dt={}", time.delta_secs());
    return ;

}

fn iterate_melee_damage(
    time: Res<Time>,
    mut q_player_rats: Query<(&mut RatPilot, &Transform, &mut ExternalForce, &mut Rat), (With<Rat>, With<PlayerAllegiance>, Without<EnemyAllegiance>)>,
    mut q_enemy_rats: Query<(&mut RatPilot, &Transform, &mut ExternalForce, &mut Rat), (With<Rat>, Without<PlayerAllegiance>, With<EnemyAllegiance>)>,
) {
    const MELEE_RANGE:f32 = 0.8;
    const MORALE_RANGE:f32 = 1.2;
    const DAMAGE_FACTOR:f32 = 5.0;
    for mut player_rat in &mut q_player_rats {
        for mut enemy_rat in &mut q_enemy_rats {
            if (player_rat.1.translation - enemy_rat.1.translation).length() <= MELEE_RANGE {
                enemy_rat.3.health.try_subtract(player_rat.2.force.length()*DAMAGE_FACTOR);
                player_rat.3.health.try_subtract(player_rat.2.force.length()*DAMAGE_FACTOR);
                RatMoods::on_lose_health(&mut enemy_rat.3);
                RatMoods::on_lose_health(&mut player_rat.3);
                println!("healthmalus!");
            }
            if (player_rat.1.translation - enemy_rat.1.translation).length() <= MORALE_RANGE {
                enemy_rat.3.morale.try_subtract(player_rat.2.force.length()*DAMAGE_FACTOR*0.3);
                player_rat.3.morale.try_subtract(player_rat.2.force.length()*DAMAGE_FACTOR*0.3);
                RatMoods::on_lose_morale(&mut enemy_rat.3);
                RatMoods::on_lose_morale(&mut player_rat.3);
                println!("moralemalus!{}", player_rat.2.force.length()*DAMAGE_FACTOR*0.3);
            }
        }
    }

}

/// use usr inputs to move cursors!
fn walk_cursors(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q_player_cursor: Single<&mut Transform, With<PlayerCursor>>,
    mut q_enemy_cursor: Single<&mut Transform, (With<EnemyCursor>, Without<PlayerCursor>)>,
) {

    // PLAYER HANDLING
    let mut player_move_req = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { player_move_req.x += 1.0; }
    if keys.pressed(KeyCode::KeyS) { player_move_req.x -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { player_move_req.z -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { player_move_req.z += 1.0; }
    // ENEMY HANDLING
    let mut enemy_move_req = Vec3::ZERO;
    if keys.pressed(KeyCode::ArrowUp)    { enemy_move_req.x += 1.0; }
    if keys.pressed(KeyCode::ArrowDown)  { enemy_move_req.x -= 1.0; }
    if keys.pressed(KeyCode::ArrowLeft)  { enemy_move_req.z -= 1.0; }
    if keys.pressed(KeyCode::ArrowRight) { enemy_move_req.z += 1.0; }
    // PROCESS REQUESTS
    if (player_move_req != Vec3::ZERO) {
        player_move_req = player_move_req.normalize() * CURSOR_SPEED * time.delta_secs();
    }
    if (enemy_move_req != Vec3::ZERO) {
        enemy_move_req = enemy_move_req.normalize() * CURSOR_SPEED * time.delta_secs();
    }
    // APPLY MOTIONS
    q_player_cursor.translation += player_move_req;
    q_enemy_cursor.translation += enemy_move_req;

}




pub trait TeamCursor {}

#[derive(Component)]
pub struct PlayerCursor;
impl TeamCursor for PlayerCursor {}

#[derive(Component)]
pub struct EnemyCursor;
impl TeamCursor for EnemyCursor {}

fn write_instructions_to_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Instructions"),
        Text::new(
            "Use WASD to move your cursor.\n\
             Your rats will follow your cursor."
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(120),
            ..default()
        },
    ));
}

/// places both teams' cursors into the world
fn setup_cursors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // PLAYER CURSOR
    commands
        .spawn(Name::new("Player Cursor"))
        .insert(Transform::from_xyz(0.0, -0.4, -2.0)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                std::f32::consts::FRAC_PI_4,
                0.0,
                std::f32::consts::FRAC_PI_4)))
        .insert(Mesh3d(meshes.add(Cuboid::new(
            0.3,
            0.3,
            0.3,
        ))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.7, 0.8), // CYAN for player
            cull_mode: None,
            ..default()
        })))
        .insert(PlayerCursor);

    // ENEMY CURSOR
    commands
        .spawn(Name::new("Enemy Cursor"))
        .insert(Transform::from_xyz(0.0, -0.4, 2.0)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                std::f32::consts::FRAC_PI_4,
                std::f32::consts::PI,
                std::f32::consts::FRAC_PI_4)))
        .insert(Mesh3d(meshes.add(Cuboid::new(
            0.3,
            0.3,
            0.3,
        ))))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2), // RED for enemy
            cull_mode: None,
            ..default()
        })))
        .insert(EnemyCursor);
    return;

}

/// places rats into the world
fn setup_rats(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
    camera_settings: Res<CameraSettings>,
) {
    const PLAYER_RAT_COLOR:Color = Color::srgb(0.40, 0.53, 0.55);
    const ENEMY_RAT_COLOR:Color = Color::srgb(0.73, 0.36, 0.36);

    const COLUMN_LENGTH:i32 = 6;
    const COLUMN_COUNT:i32 = 5;
    const SEPARATION:f32 = 0.4;
    let player_minimum_corner:Vec3 = Vec3::new(-2.0, 4.0, -3.5);

    /* Create the RATS */
    // PLAYER TEAM
    let mut rat_index = 0;
    let mut transform = Transform::default(); // TODO do we have to allocate this immediately, or can we wait until the loop?
    transform.scale *= 0.3; // TODO make a constant!
    for rat_in_row in 0..COLUMN_COUNT /* row length */ {
        for rat_in_column in 0..COLUMN_LENGTH {
            // CALCULATE NECESSARY VALUES FOR THIS STEP
            rat_index += 1;
            transform.translation = player_minimum_corner.clone();
            transform.translation.x += rat_in_column as f32 * SEPARATION;
            transform.translation.z += rat_in_row as f32 * SEPARATION;
            // SPAWN THE RAT!
            commands
                .spawn(RigidBody::Dynamic)
                .insert(Name::new(format!("Rat {}", rat_index)))
                .insert(Collider::cuboid(
                    transform.scale.x/2.0,
                    transform.scale.y/2.0,
                    transform.scale.z/2.0,
                )) // .
                .insert(Restitution::coefficient(1.0))
                .insert(Friction::coefficient(0.9))
                .insert(transform)
                .insert(GravityScale(GRAVITY))
                .insert(ExternalForce {
                    force: Vec3::ZERO,
                    torque: Vec3::ZERO,
                })
                .insert(ExternalImpulse {
                    impulse: Vec3::ZERO,
                    torque_impulse: Vec3::ZERO,
                })
                .insert(Mesh3d(meshes.add(Cuboid::new(
                    transform.scale.x,
                    transform.scale.y,
                    transform.scale.z,
                ))))
                .insert(MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: PLAYER_RAT_COLOR,
                    cull_mode: None,
                    ..default()
                })))
                .insert(PlayerAllegiance)
                .insert(Rat::new(
                    CResource::new(0.0, 1.0),
                    CResource::new(0.0, 1.0),
                    RatMoods::FRESH,
                ))
                .insert(RatPilot::new(
                    Pid::new(
                        0.02,
                        -0.02,
                        0.01,
                        0.0,
                        0.0,
                        1.0,
                        0.1,
                        0.1,
                        1.0,
                        1.0,
                    ),

                ));

        }
    }
    
    // ENEMY TEAM
    let enemy_minimum_corner:Vec3 = Vec3::new(-2.0, 4.0, 1.5);
    
    for rat_in_row in 0..COLUMN_COUNT /* row length */ {
        for rat_in_column in 0..COLUMN_LENGTH {
            // CALCULATE NECESSARY VALUES FOR THIS STEP
            rat_index += 1;
            transform.translation = enemy_minimum_corner.clone();
            transform.translation.x += rat_in_column as f32 * SEPARATION;
            transform.translation.z += rat_in_row as f32 * SEPARATION;
            // SPAWN THE RAT!
            commands
                .spawn(RigidBody::Dynamic)
                .insert(Name::new(format!("Rat {}", rat_index)))
                .insert(Collider::cuboid(
                    transform.scale.x/2.0,
                    transform.scale.y/2.0,
                    transform.scale.z/2.0,
                )) // .
                .insert(Restitution::coefficient(1.0))
                .insert(Friction::coefficient(0.9))
                .insert(transform)
                .insert(GravityScale(GRAVITY))
                .insert(ExternalForce {
                    force: Vec3::ZERO,
                    torque: Vec3::ZERO,
                })
                .insert(ExternalImpulse {
                    impulse: Vec3::ZERO,
                    torque_impulse: Vec3::ZERO,
                })
                .insert(Mesh3d(meshes.add(Cuboid::new(
                    transform.scale.x,
                    transform.scale.y,
                    transform.scale.z,
                ))))
                .insert(MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: ENEMY_RAT_COLOR,
                    cull_mode: None,
                    ..default()
                })))
                .insert(EnemyAllegiance)
                .insert(Rat::new(
                    CResource::new(0.0, 1.0),
                    CResource::new(0.0, 1.0),
                    RatMoods::FRESH,
                ))
                .insert(RatPilot::new(
                    Pid::new(
                        0.02,
                        -0.02,
                        0.01,
                        0.0,
                        0.0,
                        1.0,
                        0.1,
                        0.1,
                        1.0,
                        1.0,
                    ),

                ));

        }
    }



}


