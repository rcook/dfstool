use crate::u18;

u18!(Address);

impl Address {
    pub const ZERO: Self = Self(0);
}
