

use bevy::prelude::{Component, Transform, Mesh, Vec3};
use crate::rat::Rat;
use crate::resources::CResource;

/// Attach an <code>Allegiance</code> component to rats to let them remember which team they're on!
#[derive(Component)]
pub struct Allegiance {
    pub team : Team,
}

pub struct Team {
    pub rats : Vec<Rat>,
    // rats_sorted_by_x : Vec<Rat>,
    // rats_sorted_by_z : Vec<Rat>
}

impl Team {
    pub fn new() -> Self {
        Team {
            rats: vec![],
            // rats_sorted_by_x: vec![],
            // rats_sorted_by_z: vec![]
        }
    }

    // // TODO call this each timestep per team!
    // pub fn sort_rats(&mut self) {
    //     // sorted by X only
    //     self.rats_sorted_by_x.sort_by(
    //         |rat_a:&Rat, rat_b:&Rat| rat_a.transform.translation.x.total_cmp(&rat_b.transform.translation.x)
    //     );
    //     // sorted by Z only
    //     self.rats_sorted_by_z.sort_by(
    //         |rat_a:&Rat, rat_b:&Rat| rat_a.transform.translation.z.total_cmp(&rat_b.transform.translation.z)
    //     );
    // }

    // pub fn get_rats_sorted_by_x(self) -> Vec<Rat> {
    //     return self.rats_sorted_by_x;
    // }
    //
    // pub fn get_rats_sorted_by_z(self) -> Vec<Rat> {
    //     return self.rats_sorted_by_z;
    // }

    /// NOTE: This leaves the list of sorted rats unsorted until next frame!
    pub fn add_rat(&mut self, rat : Rat) {
        self.rats.push(rat);
        // self.rats_sorted_by_x.push(rat);
        // self.rats_sorted_by_z.push(rat);
    }

    // TODO update to be parallelizable with the RAYON library! If I can't have a million goshdarn rats in RUST, what are we doing this for?!
    // TODO or just update the center with a few random samples. Because who cares if it's perfect! (This might be the best option, actually!)
    // returns the average position of all rats on the team, ignoring vertical Y axis.
    pub fn get_center(self) -> Vec3 {
        let mut accum_x:f32 = 0.0;
        let mut accum_z:f32 = 0.0;
        for rat in self.rats.iter() {
            accum_x += rat.transform.translation.x;
            accum_z += rat.transform.translation.z;
        }
        return Vec3::new(
            accum_x/(self.rats.len() as f32),
            0.0,
            accum_z/(self.rats.len() as f32)
        );
    }

    // TODO there is plenty of room for optimization using the sorted rat lists to narrow down
    // searches without performing math for all of them.
    // TODO this is O(N^2)! Reduce this time complexity!
    pub fn get_rats_in_range_on_team(&mut self, rat:&Rat, range:f32) {
        let range_squared:f32 = range * range;
        let mut rats_in_range: Vec<&Rat> = Vec::new();
        for other_rat in self.rats.iter() {
            if rat.dist_squared(other_rat) <= range_squared {
                rats_in_range.push(other_rat);
            }
        }
    }

    // TODO This should be heavily optimized. And redone.
    // pub fn get_rats_in_range_on_team(&mut self, focus : Rat, range : f32) -> Vec<Rat> {
    //     // X AXIS ONLY
    //     // draw a slice from the first rat in range on the X axis to the last rat in range on the X axis
    //     // first rat in range...
    //     let lo_idx_rats_in_range_on_x = self.get_rats_sorted_by_x().binary_search_by(
    //
    //     );
    //     // last rat in range...
    //
    //
    //     let rats_in_range_on_x:[Rat] = self.rats_sorted_by_x()[
    //         lo_idx_rats_in_range_on_x..hi_idx_rats_in_range_on_x
    //         ];
    //
    //     // Z AXIS ONLY
    //     // draw a slice from the first rat in range on the Z axis to the last rat in range on the Z axis
    //     // first rat in range...
    //     // TODO
    //     // last rat in range...
    //     // TODO
    //     // drawing slice...
    //     let rats_in_range_on_z:[Rat] = self.get_rats_sorted_by_z()[
    //         //lo_idx_rats_in_range_on_z..hi_idx_rats_in_range_on_z
    //         1..2 // TODO
    //         ];
    //
    //     // find the union between the slices rats_in_range_on_x, rats_in_range_on_z, and then filter
    //     // for those actually within range of the focus using Euclidean distance.
    //     let range_squared = range * range;
    //     let rats_in_range : &mut Vec<Rat> = vec![];
    //
    //     if rats_in_range_on_x.len() > rats_in_range_on_z.len()
    //         // this if-statement lets us iterate over the larger list as few times as possible in the
    //         // .find() method. Note: If I had bothered to use binary search, we would consider flipping
    //         // the condition to be "<" instead of ">".
    //     {
    //         for rat in rats_in_range_on_x {
    //             if rats_in_range_on_z.iter().find(rat) {
    //                 rats_in_range.push(rat);
    //             }
    //         }
    //     }
    //     else {
    //         for rat in rats_in_range_on_z {
    //             if rats_in_range_on_x.iter().find(rat) {
    //                 if focus.dist_squared(rat) <= range_squared {
    //                     rats_in_range.push(rat);
    //                 }
    //             }
    //         }
    //     }
    //
    //     // filter for only those rats who are actually in range using Euclidean distance now.
    //
    //
    //
    //
    // }


}