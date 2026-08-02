use fromsoftware_shared::OwnedPtr;

use crate::cs::{BlockId, FieldInsHandle, SpecialEffect};

mod module;
pub use module::*;


#[repr(C)]
pub struct ChrIns {
    _vftable: usize,
    pub field_ins_handle: FieldInsHandle,
    unk10: [u8; 0x20],
    pub chr_res: *const ChrRes, // offset 0x30
    // Both of these seemingly switch between some variation map ID and overworld ID?
    pub current_map_id: BlockId,
    pub previous_map_id: BlockId,
    unk40: [u8; 0x20],
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    unk68: [u8; 0x150],
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    unk0x1c0: [u8; 0x3E0],
    //..
}

impl ChrIns {
    /// Returns true if the first bit of the byte at ChrIns+0x1F5 is set.
    /// This bit indicates a chr is inactive/hidden (e.g. Revenant's dormant summons).
    pub fn is_hidden(&self) -> bool {
        let byte = unsafe { *(self as *const ChrIns as *const u8).add(0x1F5) };
        (byte & 0x01) != 0
    }

    /// Returns the SpecialEffect manager for this character.
    pub fn special_effect(&self) -> Option<&SpecialEffect> {
        let ptr = unsafe {
            *(self as *const ChrIns as *const *const SpecialEffect).byte_add(0x190)
        };
        unsafe { ptr.as_ref() }
    }
}

/// Character resource holding model metadata. Pointed to by ChrIns+0x30.
#[repr(C)]
pub struct ChrRes {
    unk0: [u8; 0xA8],
    /// Null-terminated UTF-16 LE model ID string (e.g. "c0010").
    pub model_id: [u16; 16],
    // TODO: rest
}

impl ChrRes {
    pub fn is_model(&self, model: &str) -> bool {
        let mut i = 0;
        for c in model.encode_utf16() {
            if self.model_id.get(i).copied().unwrap_or(0) != c {
                return false;
            }
            i += 1;
        }
        self.model_id.get(i).copied().unwrap_or(0) == 0
    }
}

#[repr(C)]
pub struct ChrCtrl {
    unk0: [u8; 0xf0],
    pub flags: u8,
    //..
}

#[repr(C)]
pub struct PlayerGameData {
    unk0: [u8; 0x9C],
    pub character_name: [u16; 17],
    //..
}

#[repr(C)]
pub struct PlayerIns {
    pub chr_ins: ChrIns,
    pub player_game_data: *const PlayerGameData,
    _unk: [u8; 0x1F8], // 0x7a0 total - 0x5a0 (sizeof ChrIns)
}
