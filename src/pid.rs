/// A common industrial control system. See the <code>update()</code> for the primary faculties of this class.
/// <br><strong>Usage:</strong> Please don't use this for actual industrial control -- I'm not <em>that</em> good at Rust!
pub struct Pid {
    p_gain : f32,
    i_gain : f32,
    d_gain : f32,
    set_point : f32,
    curr_error : f32,
    prev_error : f32,
    p_max : f32,
    i_max : f32,
    d_max : f32,
    i_accumulation : f32,
    i_saturation : f32,
    max : f32,
    just_refreshed : bool,
}

impl Pid {
    pub fn new(
        p_gain:f32,
        i_gain:f32,
        d_gain:f32,
        set_point:f32,
        curr_error:f32,
        p_max : f32,
        i_max : f32,
        i_saturation : f32,
        d_max : f32,
        max : f32,
    ) -> Self {
        Self {
            p_gain,
            i_gain,
            d_gain,
            set_point,
            curr_error,
            prev_error : curr_error, // keep deriv component stable to start with
            p_max,
            i_max,
            d_max,
            i_accumulation : 0.0, // integral component does not start as activated.
            i_saturation,
            max,
            just_refreshed : true,
        }
    }
    pub fn copy (&self) -> Self {
        Self {
            p_gain : self.p_gain,
            i_gain : self.i_gain,
            d_gain : self.d_gain,
            set_point : self.set_point,
            curr_error : self.curr_error,
            prev_error : self.prev_error,
            p_max : self.p_max,
            i_max : self.i_max,
            d_max : self.d_max,
            i_accumulation : self.i_accumulation,
            i_saturation : self.i_saturation,
            max : self.max,
            just_refreshed : self.just_refreshed,
        }
    }

    /// TODO: Fix documentation to be more in line with native Rust and Bevy docs!
    /// Clamps a given value to within a maximum absolute distance from zero.
    /// <br><strong>Example:</strong> if <code>clamp()</code> is given a <code>max</code> of 5.0 and the given <code>val</code> is outside the interval [-5.0, 5.0], <code>clamp()</code> will return the nearest extrema, either -5.0 or 5.0.
    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>the newly clamped value</li>
    ///     </ul>
    ///     <li><strong>Parameters</strong></li>
    ///     <ul>
    ///         <li><strong><code>val</code></strong> the value to be clamped.</li>
    ///         <li><strong><code>max</code></strong> the positive bound on the interval the given <code>val</code> is clamped to.</li>
    ///     </ul>
    /// </ul>
    fn clamp(val:f32, max:f32) -> f32 {
        if val > max {
            return max;
        }
        else if val < -max {
            return -max;
        }
        return val;
    }

    /// Returns the PID's control output, given the current process variable's value and the time since the PID was last updated.
    /// <br><strong>Usage:</strong> Call this function each time step on the system's process variable, and use this function's output to control that system.
    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>the PID's control output</li>
    ///     </ul>
    ///     <li><strong>Parameters</strong>
    ///     <ul>
    ///         <li><strong><code>process_value</code></strong> the input value representing the system this PID is supposed to control.</li>
    ///         <li><strong><code>delta_t</code></strong> the time since this function was last called, from the perspective of the system we're controlling.
    ///     </ul>
    /// </ul>
    pub fn update(
        &mut self,
        process_value:f32,
        delta_t:f32,
        //debug: bool
    ) -> f32 {
        self.prev_error = self.curr_error;
        self.curr_error = process_value - self.set_point; // TODO check signs, deltas, &c.

        // "P" PROPORTIONAL COMPONENT
        let p_component:f32 = self.curr_error;

        // "I" INTEGRAL COMPONENT
        // accumulate the internal integral component, not surpassing saturation
        self.i_accumulation = Pid::clamp(
            self.i_accumulation + self.curr_error * delta_t,
            self.i_saturation
        );
        let i_component:f32 = self.i_accumulation;

        // "D" DERIVATIVE COMPONENT
        // forgive wherever self.prev_error was if we just refreshed :)
        // NOTE: we actually have no good way to correct this. It's probably fine if we...
        // TODO: consider removing self.just_refreshed, but leave .refresh()!
        if self.just_refreshed {
            self.prev_error = self.curr_error;
            self.just_refreshed = false;
        }

        let mut d_component:f32 = 0.0;
        if delta_t > 0.0 {
            d_component = (self.curr_error - self.prev_error) / delta_t;
        }

        let p = Pid::clamp(p_component * self.p_gain, self.p_max);
        let i = Pid::clamp(i_component * self.i_gain, self.i_max);
        let d = Pid::clamp(d_component * self.d_gain, self.d_max);

        // SUMMATION
        let sum:f32 = Pid::clamp(p + i + d, self.max);

        // DIAGNOSTICS
        // if debug {
        //     println!(
        //         "p:{}\n\
        //         i:{}\n\
        //         d:{}\n\
        //         =>{}",
        //             p, i, d, sum);
        // }
        return sum;
    }

    /// sets the integral component to 0.
    /// Call this before <code>update()</code> for intended results.
    pub fn refresh(&mut self) {
        self.just_refreshed = true;
        self.i_accumulation = 0.0;
    }


}