use shared::OwnedPtr;
use crate::cs::FieldArea;

#[repr(C)]
pub struct MoveMapStep {
    unk0: [u8; 0xF8],
    pub field_area: OwnedPtr<FieldArea>,
    unk100: [u8; 0x30],
    pub debug_pause: bool,
    unk131: [u8; 0x2FF],
    pub toggle_move_map: bool,
}

// SAFETY: MoveMapStep lives in game memory. Raw pointers within FieldArea/WorldInfoOwner
// are never accessed from Rust across threads — only debug_pause and toggle_move_map
// (bool fields) are read/written, and the game engine manages the struct's lifetime.
unsafe impl Sync for MoveMapStep {}
unsafe impl Send for MoveMapStep {}
