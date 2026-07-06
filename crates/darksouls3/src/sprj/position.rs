use std::ops::{Add, Sub};

/// Represents a position in havok physics space.
/// Layout-compatible with eldenring::position::HavokPosition.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HavokPosition(pub f32, pub f32, pub f32, pub f32);

impl HavokPosition {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 0.0)
    }
}

impl Sub for HavokPosition {
    type Output = PositionDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        PositionDelta(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<PositionDelta> for HavokPosition {
    type Output = Self;

    fn add(self, rhs: PositionDelta) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, 0.0)
    }
}

impl Sub<PositionDelta> for HavokPosition {
    type Output = Self;

    fn sub(self, rhs: PositionDelta) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, 0.0)
    }
}

/// Represents a displacement applicable to position types.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionDelta(pub f32, pub f32, pub f32);
