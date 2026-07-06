use shared::OwnedPtr;

#[repr(C)]
pub struct MoveMapStep {
    pub vftable: usize,
    pub unk0: [u8; 0x120],
    pub debug_pause: bool,
    pub unk1: [u8; 0x38F],
    pub toggle_move_map: bool,
}

// SAFETY: MoveMapStep lives in game memory. Raw pointers within FieldArea/WorldInfoOwner
// are never accessed from Rust across threads — only debug_pause and toggle_move_map
// (bool fields) are read/written, and the game engine manages the struct's lifetime.
unsafe impl Sync for MoveMapStep {}
unsafe impl Send for MoveMapStep {}
