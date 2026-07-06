use super::BlockId;

use bitfield::bitfield;

use std::fmt::Display;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FieldInsType {
    Hit = 0,
    Chr = 1,
    Obj = 2,
    Bullet = 3,
    Geom = 4,
    ReplayGhost = 5,
    ReplayEnemy = 6,
    Map = 7,
    HitGeom = 8,
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    /// Used to reference a specific FieldIns managed by its respective (external) domain.
    pub struct FieldInsSelector(u32);
    impl Debug;

    /// The index within the container.
    pub index, _: 19, 0;
    _, set_index: 19, 0;

    /// The container for this FieldIns, used to determine which ChrSet to use.
    pub container, _: 27, 20;
    _, set_container: 27, 20;

    /// The type of FieldIns, used to determine the container type.
    _field_ins_type, set_field_ins_type: 31, 28;
}

impl FieldInsSelector {
    pub fn from_parts(field_ins_type: FieldInsType, container: u32, index: u32) -> Self {
        let mut selector = FieldInsSelector(0);
        selector.set_field_ins_type(field_ins_type as u32);
        selector.set_container(container);
        selector.set_index(index);
        selector
    }

    pub fn field_ins_type(&self) -> Option<FieldInsType> {
        match self._field_ins_type() {
            0 => Some(FieldInsType::Hit),
            1 => Some(FieldInsType::Chr),
            2 => Some(FieldInsType::Obj),
            3 => Some(FieldInsType::Bullet),
            4 => Some(FieldInsType::Geom),
            5 => Some(FieldInsType::ReplayGhost),
            6 => Some(FieldInsType::ReplayEnemy),
            7 => Some(FieldInsType::Map),
            8 => Some(FieldInsType::HitGeom),
            _ => None,
        }
    }
}

/// Used throughout the game engine to refer to characters, geometry, bullets, hits and more.
#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldInsHandle {
    pub selector: FieldInsSelector,
    pub block_id: BlockId,
}

impl FieldInsHandle {
    pub fn is_empty(&self) -> bool {
        self.selector.0 == u32::MAX
    }
}

impl Display for FieldInsHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "FieldIns(None)")
        } else {
            write!(
                f,
                "FieldIns({}, {}, {})",
                self.block_id,
                self.selector.container(),
                self.selector.index()
            )
        }
    }
}
