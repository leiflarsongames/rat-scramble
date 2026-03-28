use std::future::poll_fn;
use std::ops::Deref;
use crate::rat::Rat;
// Note: I have basically given up on implementing the state pattern

const TO_DOWN_HEALTH: f32 = 0.0;
const TO_FRESH_HEALTH: f32 = 0.5;
const TO_SPOOKED_MORALE: f32 = 0.0;
const TO_FRESH_MORALE: f32 = 0.5;

pub enum RatMoods {
    FRESH,
    SPOOKED,
    DOWN,
}

pub fn on_exit(ctx:&mut Rat) {
    // TODO uncomment!
    // if ctx.mood == RatMoods::SPOOKED {
    //     ctx.pilot.set_active(true);
    // }
}

pub fn on_enter(ctx:&mut Rat) {
    // TODO uncomment!
    // if ctx.mood == RatMoods::SPOOKED {
    //     ctx.pilot.set_active(false);
    // }
}

// TODO ask Dr. Berrier what on Earth about a fixed size vtable would prohibit the construction of a dyn compatible trait and why did that happen here? like- what?
pub trait RatMoodOperations {
    /// Call me on a rat whenever they gain health.
    /// returns whether a change of state occurred.
    fn on_gain_health(ctx: &mut Rat) -> bool;
    /// Call me on a rat whenever they lose health.
    /// returns whether a change of state occurred.
    fn on_lose_health(ctx: &mut Rat) -> bool;

    /// Call me on a rat whenever they gain morale.
    /// returns whether a change of state occurred.
    fn on_gain_morale(ctx: &mut Rat) -> bool;
    /// Call me on a rat whenever they lose morale.
    /// returns whether a change of state occurred.
    fn on_lose_morale(ctx: &mut Rat) -> bool;
}

// TODO reimplement this without a bunch of switch cases t-t
// TODO try to adhere to the state pattern
impl RatMoodOperations for RatMoods {
    fn on_gain_health(ctx: &mut Rat) -> bool {
        return match ctx.deref() {
            FRESH => FRESH_on_gain_health(ctx),
            SPOOKED => SPOOKED_on_gain_health(ctx),
            DOWN => DOWN_on_gain_health(ctx),
        }
    }

    fn on_lose_health(ctx: &mut Rat) -> bool {
        return match ctx.deref() {
            FRESH => FRESH_on_lose_health(ctx),
            SPOOKED => SPOOKED_on_lose_health(ctx),
            DOWN => DOWN_on_lose_health(ctx),
        }
    }

    fn on_gain_morale(ctx: &mut Rat) -> bool {
        return match ctx.deref() {
            FRESH => FRESH_on_gain_morale(ctx),
            SPOOKED => SPOOKED_on_gain_morale(ctx),
            DOWN => DOWN_on_gain_morale(ctx),
        }
    }

    fn on_lose_morale(ctx: &mut Rat) -> bool {
        return match ctx.deref() {
            FRESH => FRESH_on_lose_morale(ctx),
            SPOOKED => SPOOKED_on_lose_morale(ctx),
            DOWN => DOWN_on_lose_morale(ctx),
        }
    }
}

// TODO implement these in some polymorphic way instead of oh god what is this

// FRESH
fn FRESH_on_gain_health(ctx: &mut Rat) -> bool { false }
fn FRESH_on_lose_health(ctx: &mut Rat) -> bool {
    if ctx.health.get_value() <= TO_DOWN_HEALTH {
        on_exit(ctx);
        ctx.mood = RatMoods::DOWN;
        on_enter(ctx);
        return true;
    }
    return false;
}
fn FRESH_on_gain_morale(ctx: &mut Rat) -> bool { false }
fn FRESH_on_lose_morale(ctx: &mut Rat) -> bool {
    if ctx.morale.get_value() <= TO_SPOOKED_MORALE {
        on_exit(ctx);
        ctx.mood = RatMoods::SPOOKED;
        on_enter(ctx);
        return true;
    }
    return false;
}

//SPOOKED
fn SPOOKED_on_gain_health(ctx: &mut Rat) -> bool { false }
fn SPOOKED_on_lose_health(ctx: &mut Rat) -> bool {
    if ctx.health.get_value() <= TO_DOWN_HEALTH {
        on_exit(ctx);
        ctx.mood = RatMoods::DOWN;
        on_enter(ctx);
        return true;
    }
    return false;
}
fn SPOOKED_on_gain_morale(ctx: &mut Rat) -> bool {
    if ctx.morale.get_value() >= TO_FRESH_MORALE {
        on_exit(ctx);
        ctx.mood = RatMoods::FRESH;
        on_enter(ctx);
        return true;
    }
    return false;
}
fn SPOOKED_on_lose_morale(ctx: &mut Rat) -> bool { false }

//DOWN
fn DOWN_on_lose_health(ctx: &mut Rat) -> bool { return false; }
fn DOWN_on_gain_health(ctx: &mut Rat) -> bool {
    if ctx.health.get_value() >= TO_FRESH_HEALTH {
        on_exit(ctx);
        if ctx.morale.get_value() >= TO_FRESH_MORALE {
            ctx.mood = RatMoods::FRESH;
        }
        else {
            ctx.mood = RatMoods::SPOOKED;
        }
        on_enter(ctx);
        return true;
    }
    return false;
}

fn DOWN_on_gain_morale(ctx: &mut Rat) -> bool { return false; }
fn DOWN_on_lose_morale(ctx: &mut Rat) -> bool { return false; }
