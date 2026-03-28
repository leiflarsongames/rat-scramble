use bevy::math::Vec3;
use crate::rat::Rat;

// According to
pub trait RatMood : Singleton {
    /// returns whether a change of state occurred.
    fn on_gain_health(self, ctx: &mut Rat) -> bool;
    /// returns whether a change of state occurred.
    fn on_lost_health(self, ctx: &mut Rat) -> bool;

    /// returns whether a change of state occurred.
    fn on_gain_morale(self, ctx: &mut Rat) -> bool;
    /// returns whether a change of state occurred.
    fn on_lose_morale(self, ctx: &mut Rat) -> bool;
}

pub enum RatMoods {
    FRESH(Fresh),
    SPOOKED(Spooked),
    DOWN(Down),
}

const TO_DOWN_HEALTH: f32 = 0.0;
const TO_FRESH_HEALTH: f32 = 0.5;
const TO_SPOOKED_MORALE: f32 = 0.0;
const TO_FRESH_MORALE: f32 = 0.5;

// TODO make these structs NOT public?

pub struct Fresh;
pub struct Spooked;
pub struct Down;

pub trait Singleton { fn get_instance() -> &'static Option<Box<dyn RatMood>>; }
impl Singleton for Fresh { fn get_instance() -> RatMoods { RatMoods( Fresh ) } }
impl Singleton for Spooked { fn get_instance() -> RatMoods Option<Box<dyn RatMood>>; }
impl Singleton for Down { fn get_instance() -> &'static Option<Box<dyn RatMood>>; }

impl RatMood for Fresh {

    fn on_gain_health(self, ctx: &mut Rat) -> bool { return false; }
    fn on_lost_health(self, ctx: &mut Rat) -> bool {
        if ctx.health.get_value() <= TO_DOWN_HEALTH {
            ctx.mood = Down::get_instance();
            return true;
        }
        return false;
    }

    fn on_gain_morale(&self, ctx: &Rat) -> bool { return false; }
    fn on_lose_morale(ctx: &mut Rat) -> bool {
        if ctx.morale.get_value() <= TO_SPOOKED_MORALE {
            ctx.mood = Spooked::get_instance();
            return true;
        }
        return false;
    }
}

impl RatMood for Spooked {
    fn on_gain_health(ctx: &mut Rat) -> bool { return false; }
    fn on_lost_health(ctx: &mut Rat) -> bool {
        if ctx.health.get_value() <= TO_DOWN_HEALTH {
            ctx.mood = Down::get_instance();
            return true;
        }
        return false;
    }

    fn on_gain_morale(ctx: &mut Rat) -> bool {
        if ctx.morale.get_value() >= TO_FRESH_MORALE {
            ctx.mood = Fresh::get_instance();
            return true;
        }
        return false;
    }
    fn on_lose_morale(ctx: &mut Rat) -> bool { return false; }
}



impl RatMood for Down {
    fn on_lost_health(ctx: &mut Rat) -> bool { return false; }
    fn on_gain_health(ctx: &mut Rat) -> bool {
        if ctx.health.get_value() >= TO_FRESH_HEALTH {
            if ctx.morale.get_value() >= TO_FRESH_MORALE {
                ctx.mood = Fresh::get_instance();
            }
            else {
                ctx.mood = Spooked::get_instance();
            }
            return true;
        }
        return false;
    }

    fn on_gain_morale(ctx: &mut Rat) -> bool { return false; }
    fn on_lose_morale(ctx: &mut Rat) -> bool { return false; }
}


