use shared::OwnedPtr;

#[repr(C)]
pub struct MoveMapStep {
    pub vftable: usize,
    pub unk0: [u8; 0x1ED], // vftable=8 bytes, so pad = 0x1f5 - 0x08 = 0x1ed
    pub debug_pause: bool,      // +0x1f5
    pub toggle_move_map: bool, // +0x1f6
}

// SAFETY: MoveMapStep lives in game memory. Raw pointers within FieldArea/WorldInfoOwner
// are never accessed from Rust across threads — only debug_pause and toggle_move_map
// (bool fields) are read/written, and the game engine manages the struct's lifetime.
unsafe impl Sync for MoveMapStep {}
unsafe impl Send for MoveMapStep {}
