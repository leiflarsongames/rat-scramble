use std::future::poll_fn;
use crate::rat::Rat;

// TODO remove if unused!
// pub trait Singleton { fn get_instance() -> &'static Self; }

const TO_DOWN_HEALTH: f32 = 0.0;
const TO_FRESH_HEALTH: f32 = 0.5;
const TO_SPOOKED_MORALE: f32 = 0.0;
const TO_FRESH_MORALE: f32 = 0.5;

pub enum RatMoods {
    FRESH(fn(&mut Rat)->bool),
    SPOOKED(fn(&mut Rat)->bool),
    DOWN(fn(&mut Rat)->bool),
}

// I'm absolutely grasping at like literally anything here to refer to these different functions 
// without a trait that's dyn compatible. Something, something... vtables? That's probably really 
// darn important.
const RMOPS:Vec<Any> = vec![Fresh, Spooked, ]


impl RatMoods {
    pub fn operate(self, ctx:&mut Rat) -> bool {
        // I CAVED I'M A FRAUD THERE I SAID IT IM A FRAUD I DIDN'T IMPLEMENT THE STATE PATTERN CRUCIFY ME
        // TODO try implementing the state pattern harder
        // TODO to elaborate... couple this bs looser my man this ain't it
        match self {
            RatMoods::FRESH(F) => { Fresh.F(ctx) }
            RatMoods::SPOOKED(F) => { Spooked.F(ctx) }
            RatMoods::DOWN(F) => { Down.F(ctx) }
        }
    }

    // pub fn operate(self, ctx:&mut Rat) -> bool {
    //     // I CAVED I'M A FRAUD THERE I SAID IT IM A FRAUD I DIDN'T IMPLEMENT THE STATE PATTERN CRUCIFY ME
    //     // TODO try implementing the state pattern harder
    //     // TODO to elaborate... couple this bs looser my man this ain't it
    //     match self {
    //         RatMoods::FRESH(F) => { F(Fresh, ctx) }
    //         RatMoods::SPOOKED(F) => { F(Spooked, ctx) }
    //         RatMoods::DOWN(F) => { F(Down, ctx) }
    //     }
    // }
}



pub trait RatMoodOperations {
    /// Call me on a rat whenever they gain health.
    /// returns whether a change of state occurred.
    fn on_gain_health(ctx: &mut Rat) -> bool;
    /// Call me on a rat whenever they lose health.
    /// returns whether a change of state occurred.
    fn on_lost_health(ctx: &mut Rat) -> bool;

    /// Call me on a rat whenever they gain morale.
    /// returns whether a change of state occurred.
    fn on_gain_morale(ctx: &mut Rat) -> bool;
    /// Call me on a rat whenever they lose morale.
    /// returns whether a change of state occurred.
    fn on_lose_morale(ctx: &mut Rat) -> bool;
}

pub struct Fresh;

impl RatMoodOperations for Fresh {
    fn on_gain_health(ctx: &mut Rat) -> bool { false }
    fn on_lost_health(ctx: &mut Rat) -> bool {
        if ctx.health.get_value() <= TO_DOWN_HEALTH {
            ctx.mood = RatMoods::DOWN;
            return true;
        }
        return false;
    }

    fn on_gain_morale(ctx: &mut Rat) -> bool { false }
    fn on_lose_morale(ctx: &mut Rat) -> bool {
        if ctx.morale.get_value() <= TO_SPOOKED_MORALE {
            ctx.mood = RatMoods::SPOOKED;
            return true;
        }
        return false;
    }
}