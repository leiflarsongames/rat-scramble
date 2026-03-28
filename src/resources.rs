use bevy::prelude::Component;

// TODO fix docs to fit better with Rust's documentation

/// Short for "Continuous Resource". Represents a non-discrete depletable resource, such as Hit-Points, Thirst or Magicka.
/// <br><small><strong>NOTE:</strong> since <code>CResource</code> uses f32 (floats) internally, it would not be
/// suitable for something discretely countable like Gold or Ammunition.</small>
pub struct CResource {
    max_value: f32,
    min_value: f32,
    value: f32,
}


impl CResource {

    /// Constructs a new <code>CResource</code>.
    /// <ul>
    ///     <li><strong>Parameters</strong></li>
    ///     <ul>
    ///         <li><code>min_value</code>
    ///             the minimum value this <code>CResource</code> will accept. Any values lower than
    ///             this will be clamped up to this minimum.
    ///         </li>
    ///         <li><code>max_value</code>
    ///             the maximum value this <code>CResource</code> will accept. Any values greater than
    ///             this will be clamped down to this maximum.
    ///         </li>
    ///     </ul>
    /// </ul>
    pub fn new(min_value:f32, max_value:f32) -> Self {
        Self {
            max_value,
            min_value,
            value:max_value,
        }
    }

    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>the lowest value this <code>CResource</code> can encapsulate.</li>
    ///     </ul>
    /// </ul>
    pub fn get_min_value(&self) -> f32 {
        return self.min_value;
    }

    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>the greatest value this <code>CResource</code> can encapsulate.</li>
    ///     </ul>
    /// </ul>
    pub fn get_max_value(&self) -> f32 {
        return self.max_value;
    }

    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>current value</li>
    ///     </ul>
    /// </ul>
    pub fn get_value(&self) -> f32 {
        return self.value;
    }

    /// Adds the given <code>addend</code> to this <code>CResource</code>'s value.
    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>whether the operation did anything.</li>
    ///     </ul>
    ///     <li><Strong>Note:</strong> also returns <code>true</code> when adding 0.0</li>
    /// </ul>
    pub fn try_gain(&mut self, addend:f32) -> bool {

        if addend >= 0.0 {
            self.value += addend;
            // cap value at max_value
            if self.max_value < self.value {
                self.value = self.max_value;
            }
            return true;
        }
        return false; // do not add a negative amount with this function.
    }

    /// Subtracts the given amount. Useful for inflicting damage.
    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>whether the operation did anything.</li>
    ///     </ul>
    /// </ul>
    pub fn try_subtract(&mut self, subtrahend:f32) -> bool {
        // If there is no resource remaining
        if self.value <= self.min_value {
            return false; // refuse to subtract if at or below the minimum value
        }
        // subtracting more resource than actually exists
        else {
            self.value -= subtrahend;
            // prevent self.value from dropping below the minimum value
            if self.value < self.min_value {
                self.value = self.min_value;
            }
            return true;
        }
    }

    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>
    ///             whether this <code>CResource</code> can cover the cost and still remain above
    ///             both zero and the <code>min_value</code>.
    ///         </li>
    ///     </ul>
    ///     <li><strong>See also:</strong></li>
    ///     <ul>
    ///         <li><code>can_afford_allowing_debt()</code>
    ///             allows spending past 0.0, if the minimum is low enough.
    ///         </li>
    ///     </ul>
    /// </ul>
    pub fn can_afford(self, cost:f32) -> bool {

        return self.value >= cost && self.value-self.min_value >= cost;
    }

    /// <ul>
    ///     <li><strong>Returns</strong></li>
    ///     <ul>
    ///         <li>
    ///             whether this <code>CResource</code> can cover the cost and still remain above
    ///             the <code>min_value</code>.
    ///         </li>
    ///     </ul>
    ///     <li><strong>See also:</strong></li>
    ///     <ul>
    ///         <li><code>can_afford()</code>
    ///             does not allow spending below 0.0, regardless of whether the minimum of this
    ///             <code>CResource</code> would allow it.
    ///         </li>
    ///     </ul>
    /// </ul>
    pub fn can_afford_allowing_debt(self, cost:f32) -> bool {
        return self.value-self.min_value >= cost;
    }

}
