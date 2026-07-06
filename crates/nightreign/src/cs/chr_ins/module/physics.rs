use std::ptr::NonNull;

use crate::cs::ChrIns;
use crate::position::{HavokPosition, Quaternion};

#[repr(C)]
pub struct CSChrPhysicsModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x40],           // 0x10..0x50
    pub orientation: Quaternion, // 0x50
    unk60: [u8; 0x10],           // 0x60..0x70
    pub position: HavokPosition, // 0x70
    // TODO: the rest
}
