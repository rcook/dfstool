use crate::u10;

u10!(SectorSize);

impl SectorSize {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
}
