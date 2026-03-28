use bevy::prelude::{Component, Transform, Vec3};
use bevy_rapier3d::prelude::{ExternalImpulse, RigidBody};
use crate::pid::Pid;

#[derive(Component)]
pub struct RatPilot {
    active : bool,
    x_pid : Pid,
    z_pid : Pid,
}

impl RatPilot {
    pub fn new(pid : Pid) -> RatPilot {
        return RatPilot { active:true, x_pid:pid.copy(), z_pid:pid };
    }

    /// sets whether update should issue a useful command.
    pub fn set_active(&mut self, active:bool) {
        if active && self.active {
            // this case involves extra work to "clear" each PID.
            self.x_pid.refresh();
            self.z_pid.refresh();
        }
        self.active = active;
    }

    /// Returns the force to be applied to the rat this tick.
    /// <br><small>NOTE: Should be called for each rat each tick.</small>
    pub fn update(&mut self, set_point:Vec3, current_location:Vec3, delta_t:f32, /*debug: bool*/) -> Vec3 {
        if self.active {
            return Vec3::new(
                self.x_pid.update(
                    set_point.x - current_location.x, 
                    delta_t, 
                    // debug
                ),
                0.0,
                self.z_pid.update(
                    set_point.z - current_location.z, 
                    delta_t, 
                    // false
                ),
            );
        }
        else {
            return Vec3::ZERO; // give no force commands.
        }
    }


}

