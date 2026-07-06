use std::ptr::NonNull;

use shared::OwnedPtr;

use super::ChrIns;

#[repr(C)]
/// Source of name: copied from ER
pub struct ChrInsModuleContainer {
    _action_flag: u64,
    _behavior_script: u64,
    pub time_act: OwnedPtr<SprjChrTimeActModule>,
    pub data: OwnedPtr<ChrDataModule>,
    _resist: u64,
    _behavior: u64,
    _behavior_sync: u64,
    _ai: u64,
    _super_armor: u64,
    _toughness: u64,
    _talk: u64,
    _event: u64,
    _magic: u64,
    pub physics: OwnedPtr<SprjChrPhysicsModule>,
    _fall: u64,
    _ladder: u64,
    _action_request: u64,
    _throw: u64,
    _hit_stop: u64,
    _damage: u64,
    _material: u64,
    _knock_back: u64,
    _sfx: u64,
    _behavior_data: u64,
    _data_ex: u64,
    _ceremony: u64,
    _unkd0: u64,
    _model_param_modifier: u64,
    _se_ai_sound: u64,
    _dripping: u64,
    _accumulate_fire: u64,
    _foot_effect: u64,
    _sword_arts: u64,
    _wet: u64,
    _auto_homing: u64,
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrModuleBase {
    _vftable: usize,
    pub owner: NonNull<ChrIns>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrDataModule {
    pub super_chr_module: ChrModuleBase,
    _unk10: usize,
    _unk18: usize,
    _unk20: u32,
    _unk24: u32,
    _unk28: usize,
    _unk30: [u8; 0xa8],
    pub hp: i32,
    pub max_hp: i32,
    pub base_hp: i32,
    pub fp: i32,
    pub max_fp: i32,
    pub base_fp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub base_stamina: i32,
    _unkfc: u32,
    _unk100: u32,
    _unk104: u32,
    _unk108: [u8; 0x28],
    pub name: [u16; 8],
    _unk140: [u8; 0x80],
    pub debug_flags: u8,
    _unk181: [u8; 0x7],
}

impl ChrDataModule {
    /// Returns the character ID string for this character, of the form `c1234`.
    pub fn id(&self) -> String {
        let len = self
            .name
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.name.len());
        String::from_utf16(&self.name[..len]).unwrap_or_else(|_| "<invalid>".to_string())
    }
}

/// DS3 equivalent of ER's CSChrPhysicsModule.
/// Layout differs from ER: orientation stored as pitch/yaw floats rather than a quaternion.
#[repr(C)]
pub struct SprjChrPhysicsModule {
    _vftable: usize,
    pub owner: NonNull<ChrIns>,
    _unk10: [u8; 0x60], // 0x10..0x70
    pub pitch: f32,     // +0x70 — positive = looking down, negative = looking up
    pub yaw: f32,       // +0x74 — 0 = towards -z (same convention as ER)
    _unk78: [u8; 0x08], // 0x78..0x80
    pub position: [f32; 3], // +0x80 — XYZ world position
}

/// DS3 equivalent of ER's CSChrTimeActModule. Layout is identical; only the name prefix differs.
#[repr(C)]
pub struct SprjChrTimeActModule {
    _vftable: usize,
    pub owner: NonNull<ChrIns>,
    _hvk_anim: usize,
    _chr_tae_anim_event: usize,
    /// Circular buffer of animations to play.
    pub anim_queue: [SprjChrTimeActModuleAnim; 10],
    /// Index of the next animation to play or update.
    pub write_idx: u32,
    /// Index of the last animation played or updated.
    pub read_idx: u32,
    _unkc8: u32,
    _unkcc: u32,
    _unkd0: u32,
    _unkd4: u32,
}

#[repr(C)]
pub struct SprjChrTimeActModuleAnim {
    pub anim_id: i32,
    pub play_time_from: f32,
    pub play_time_to: f32,
    pub anim_length: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x1c8, size_of::<ChrDataModule>());
    }
}
